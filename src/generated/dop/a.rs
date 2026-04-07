#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopActivevalueParmopActive {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopActivevalueDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopActivevalueSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopActivevalue {
    pub base: crate::core::types::NodeBase,
}

impl DopActivevalue {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_active(mut self, val: f32) -> Self {
        self.base.params.insert(
            "active".to_string(),
            crate::core::types::ParamValue::Float(val),
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

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_active(mut self, val: DopActivevalueParmopActive) -> Self {
        self.base.params.insert(
            "parmop_active".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_active_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_active".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopActivevalueDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopActivevalueSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopActivevalue {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "activevalue"
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
pub struct DopAffector {
    pub base: crate::core::types::NodeBase,
}

impl DopAffector {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_affector_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("affector{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affector_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("affector{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affected_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("affected{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affected_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("affected{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAffector {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "affector"
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
pub enum DopAgentarcingcliplayerTurndirection {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Clone)]
pub struct DopAgentarcingcliplayer {
    pub base: crate::core::types::NodeBase,
}

impl DopAgentarcingcliplayer {
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

    /// Connects to input 0: "Stream to Apply Clips to"
    pub fn set_input_stream_to_apply_clips_to(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Stream to Apply Clips to" and specifies the output index of the target node.
    pub fn set_input_stream_to_apply_clips_to_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_turnradius_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("turnradius_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_turnradius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("turnradius_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_turndirection_inst(
        mut self,
        index1: usize,
        val: DopAgentarcingcliplayerTurndirection,
    ) -> Self {
        self.base.params.insert(
            format!("turndirection_{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_turndirection_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("turndirection_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("clipname_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("clipname_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAgentarcingcliplayer {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "agentarcingcliplayer"
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
pub enum DopAgentcliplayerBlendmode {
    Interpolate = 0,
    Additive = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentcliplayerTriggerinput {
    FirstTriggerInput = 0,
    SecondTriggerInput = 1,
    ThirdTriggerInput = 2,
}

#[derive(Debug, Clone)]
pub struct DopAgentcliplayer {
    pub base: crate::core::types::NodeBase,
}

impl DopAgentcliplayer {
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

    /// Connects to input 0: "Agents"
    pub fn set_input_agents(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Agents" and specifies the output index of the target node.
    pub fn set_input_agents_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Trigger"
    pub fn set_input_trigger(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Trigger" and specifies the output index of the target node.
    pub fn set_input_trigger_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Trigger"
    pub fn set_input_trigger_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Trigger" and specifies the output index of the target node.
    pub fn set_input_trigger_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Trigger"
    pub fn set_input_trigger_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Trigger" and specifies the output index of the target node.
    pub fn set_input_trigger_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_randomclipseed_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("randomclipseed_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipseed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("randomclipseed_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clippatternweight_inst(mut self, index1: usize, index2: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("clippatternweight_{}_{}", index1, index2),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clippatternweight_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.base.params.insert(
            format!("clippatternweight_{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendratio_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("blendratio_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendratio_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blendratio_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weight_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("weight_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("weight_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initialcliptime_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("initialcliptime_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initialcliptime_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("initialcliptime_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomclipoffset_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("randomclipoffset_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipoffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("randomclipoffset_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomclipoffsetseed_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("randomclipoffsetseed_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipoffsetseed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("randomclipoffsetseed_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipspeedmultiplier_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("clipspeedmultiplier_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedmultiplier_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("clipspeedmultiplier_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_locomotionspeedmultiplier_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("locomotionspeedmultiplier_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_locomotionspeedmultiplier_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("locomotionspeedmultiplier_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipspeedvariance_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("clipspeedvariance_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedvariance_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("clipspeedvariance_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipspeedseed_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("clipspeedseed_{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedseed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("clipspeedseed_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_blendinframes_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("blendinframes_{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blendinframes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blendinframes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendoutframes_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("blendoutframes_{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blendoutframes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blendoutframes_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_blendmode(mut self, val: DopAgentcliplayerBlendmode) -> Self {
        self.base.params.insert(
            "blendmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triggerinput_inst(
        mut self,
        index1: usize,
        val: DopAgentcliplayerTriggerinput,
    ) -> Self {
        self.base.params.insert(
            format!("triggerinput_{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_triggerinput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("triggerinput_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clipname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("clipname_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("clipname_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clippattern_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("clippattern_{}_{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clippattern_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("clippattern_{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transformgroup_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("transformgroup_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transformgroup_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("transformgroup_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localexpression_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("localexpression_{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localexpression_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("localexpression_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localweightexpression(mut self, val: &str) -> Self {
        self.base.params.insert(
            "localweightexpression".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localweightexpression_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localweightexpression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeo(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomizeclips_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("randomizeclips_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizeclips_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("randomizeclips_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetrigger_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("usetrigger_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetrigger_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("usetrigger_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_triggeronce_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("triggeronce_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_triggeronce_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("triggeronce_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setinitialcliptime_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("setinitialcliptime_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinitialcliptime_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("setinitialcliptime_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomizecliptime_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("randomizecliptime_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizecliptime_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("randomizecliptime_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overrideclipspeed_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("overrideclipspeed_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideclipspeed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("overrideclipspeed_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomizeclipspeed_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("randomizeclipspeed_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizeclipspeed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("randomizeclipspeed_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselocal_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("uselocal_{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocal_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("uselocal_{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselocalweight(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uselocalweight".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocalweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uselocalweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAgentcliplayer {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "agentcliplayer"
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
pub enum DopAgentlookatTargettype {
    Position = 0,
    Object = 1,
    Points = 2,
    Agents = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentlookatGeometrysource {
    Sop = 0,
    DopData = 1,
    UseFirstContextGeometry = 2,
    UseSecondContextGeometry = 3,
    UseThirdContextGeometry = 4,
    UseFourthContextGeometry = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentlookatHozlimitmode {
    SetValue = 0,
    SetFromAttribute = 1,
    ScaleByAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentlookatVertlimitmode {
    SetValue = 0,
    SetFromAttribute = 1,
    ScaleByAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentlookatDurationtimemode {
    SetValue = 0,
    SetFromAttribute = 1,
    ScaleByAttribute = 2,
}

#[derive(Debug, Clone)]
pub struct DopAgentlookat {
    pub base: crate::core::types::NodeBase,
}

impl DopAgentlookat {
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

    /// Connects to input 0: "Agents"
    pub fn set_input_agents(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Agents" and specifies the output index of the target node.
    pub fn set_input_agents_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_hozlimitangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hozlimitangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hozlimitangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hozlimitangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vertlimitangle(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vertlimitangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vertlimitangle_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vertlimitangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distanceweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "distanceweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distanceweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distanceweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_peripheryweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "peripheryweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_peripheryweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "peripheryweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relorientweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "relorientweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_relorientweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relorientweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relspeedweight(mut self, val: f32) -> Self {
        self.base.params.insert(
            "relspeedweight".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_relspeedweight_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relspeedweight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scoremultiplier(mut self, val: f32) -> Self {
        self.base.params.insert(
            "scoremultiplier".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scoremultiplier_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scoremultiplier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomdurationoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "randomdurationoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomdurationoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomdurationoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomdurationseed(mut self, val: f32) -> Self {
        self.base.params.insert(
            "randomdurationseed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomdurationseed_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomdurationseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_distancerange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "distancerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_distancerange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distancerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relspeedrange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "relspeedrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_relspeedrange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relspeedrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_durationtime(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "durationtime".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_durationtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "durationtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_position(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "position".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_position_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "position".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activate(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxsearchpts(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxsearchpts".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsearchpts_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxsearchpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxneighbors(mut self, val: i32) -> Self {
        self.base.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxneighbors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxneighbors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_targettype(mut self, val: DopAgentlookatTargettype) -> Self {
        self.base.params.insert(
            "targettype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targettype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targettype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometrysource(mut self, val: DopAgentlookatGeometrysource) -> Self {
        self.base.params.insert(
            "geometrysource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geometrysource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometrysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hozlimitmode(mut self, val: DopAgentlookatHozlimitmode) -> Self {
        self.base.params.insert(
            "hozlimitmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hozlimitmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hozlimitmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vertlimitmode(mut self, val: DopAgentlookatVertlimitmode) -> Self {
        self.base.params.insert(
            "vertlimitmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vertlimitmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vertlimitmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_durationtimemode(mut self, val: DopAgentlookatDurationtimemode) -> Self {
        self.base.params.insert(
            "durationtimemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_durationtimemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "durationtimemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_targetname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_objectpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "objectpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objectpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "objectpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopobject(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopobject_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dopdataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dopdataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dopdataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dopdataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_searchgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "searchgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_searchgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "searchgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_neighborgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "neighborgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_neighborgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "neighborgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_agentattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "agentattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_agentattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "agentattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hozlimitattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hozlimitattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hozlimitattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hozlimitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vertlimitattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vertlimitattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vertlimitattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vertlimitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_durationtimeattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "durationtimeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_durationtimeattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "durationtimeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchbyattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "matchbyattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchbyattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchbyattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetargetattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetargetattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetargetattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetargetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randomizedurationtime(mut self, val: bool) -> Self {
        self.base.params.insert(
            "randomizedurationtime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizedurationtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "randomizedurationtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAgentlookat {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "agentlookat"
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
pub enum DopAgentlookatapplyMintargetscoremode {
    SetValue = 0,
    SetFromAttribute = 1,
    ScaleByAttribute = 2,
}

#[derive(Debug, Clone)]
pub struct DopAgentlookatapply {
    pub base: crate::core::types::NodeBase,
}

impl DopAgentlookatapply {
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

    /// Connects to input 0: "Agents"
    pub fn set_input_agents(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Agents" and specifies the output index of the target node.
    pub fn set_input_agents_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_activate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintargetscore(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mintargetscore".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mintargetscore_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mintargetscore".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headturnstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "headturnstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headturnstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "headturnstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headweightscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "headweightscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headweightscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "headweightscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeturnstiffness(mut self, val: f32) -> Self {
        self.base.params.insert(
            "eyeturnstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeturnstiffness_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eyeturnstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeweightscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "eyeweightscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeweightscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eyeweightscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_headweightanglerange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "headweightanglerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_headweightanglerange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "headweightanglerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeweightanglerange(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "eyeweightanglerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_eyeweightanglerange_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eyeweightanglerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refdir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refup(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numjoints(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numjoints".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numjoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numjoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mintargetscoremode(mut self, val: DopAgentlookatapplyMintargetscoremode) -> Self {
        self.base.params.insert(
            "mintargetscoremode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mintargetscoremode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mintargetscoremode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_headweightramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "headweightramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_headweightramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "headweightramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_eyeweightramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "eyeweightramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_eyeweightramp_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "eyeweightramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintargetscoreattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "mintargetscoreattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mintargetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mintargetscoreattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetnameattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetnameattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetnameattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetnameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetposattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetposattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetposattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetposattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetscoreattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetscoreattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetscoreattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setnumjoints(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setnumjoints".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setnumjoints_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setnumjoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_immediateadjust(mut self, val: bool) -> Self {
        self.base.params.insert(
            "immediateadjust".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_immediateadjust_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "immediateadjust".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtargetnameattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputtargetnameattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetnameattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputtargetnameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtargetposattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputtargetposattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetposattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputtargetposattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtargetscoreattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "outputtargetscoreattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputtargetscoreattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAgentlookatapply {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "agentlookatapply"
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
pub enum DopAgentterrainadaptationTerrainsource {
    Sop = 0,
    DopData = 1,
    FirstContextGeometry = 2,
    SecondContextGeometry = 3,
    ThirdContextGeometry = 4,
    FourthContextGeometry = 5,
}

#[derive(Debug, Clone)]
pub struct DopAgentterrainadaptation {
    pub base: crate::core::types::NodeBase,
}

impl DopAgentterrainadaptation {
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

    /// Connects to input 0: "Agents"
    pub fn set_input_agents(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Agents" and specifies the output index of the target node.
    pub fn set_input_agents_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_activate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hipoffset(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hipoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipoffset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hipoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hipshiftperframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "hipshiftperframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipshiftperframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hipshiftperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_kneedampingthreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "kneedampingthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kneedampingthreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "kneedampingthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tiltangleperframe(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tiltangleperframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tiltangleperframe_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tiltangleperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mintilt(mut self, val: f32) -> Self {
        self.base.params.insert(
            "mintilt".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mintilt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mintilt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxtilt(mut self, val: f32) -> Self {
        self.base.params.insert(
            "maxtilt".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtilt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "maxtilt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidescale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidescale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lockedscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "lockedscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lockedscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lockedscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_refdir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_refup(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "refup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_terrainsource(mut self, val: DopAgentterrainadaptationTerrainsource) -> Self {
        self.base.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_terrainsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_guidecolor(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_bindgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bindgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bindgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainobject(mut self, val: &str) -> Self {
        self.base.params.insert(
            "terrainobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terrainobject_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terrainobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainsoppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "terrainsoppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terrainsoppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terrainsoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terraingroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "terraingroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terraingroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terraingroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablefootlocking(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablefootlocking".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefootlocking_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablefootlocking".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adjusthips(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adjusthips".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjusthips_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adjusthips".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limithipshift(mut self, val: bool) -> Self {
        self.base.params.insert(
            "limithipshift".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limithipshift_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limithipshift".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablekneedamping(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablekneedamping".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablekneedamping_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablekneedamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableterrainadaptation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableterrainadaptation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableterrainadaptation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableterrainadaptation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableleaning(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableleaning".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableleaning_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableleaning".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAgentterrainadaptation {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "agentterrainadaptation"
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
pub enum DopAgentterrainprojectionTerrainsource {
    Sop = 0,
    DopData = 1,
    FirstContextGeometry = 2,
    SecondContextGeometry = 3,
    ThirdContextGeometry = 4,
    FourthContextGeometry = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentterrainprojectionMode {
    DirectionVector = 0,
    UpAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentterrainprojectionSamplingmethod {
    Particle = 0,
    Foot = 1,
}

#[derive(Debug, Clone)]
pub struct DopAgentterrainprojection {
    pub base: crate::core::types::NodeBase,
}

impl DopAgentterrainprojection {
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

    /// Connects to input 0: "Agents"
    pub fn set_input_agents(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Agents" and specifies the output index of the target node.
    pub fn set_input_agents_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
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

    // --- Float3 parameters ---
    pub fn with_projectiondir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "projectiondir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_projectiondir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projectiondir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_enable(mut self, val: i32) -> Self {
        self.base.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enable_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_terrainsource(mut self, val: DopAgentterrainprojectionTerrainsource) -> Self {
        self.base.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_terrainsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: DopAgentterrainprojectionMode) -> Self {
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
    pub fn with_samplingmethod(mut self, val: DopAgentterrainprojectionSamplingmethod) -> Self {
        self.base.params.insert(
            "samplingmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplingmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "samplingmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "partgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_binddopinput2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "binddopinput2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_binddopinput2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "binddopinput2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bindgeoinput2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeoinput2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bindgeoinput2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terraingroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "terraingroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terraingroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terraingroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normalattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "normalattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_normalattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "normalattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primnumattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primnumattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primuvwattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primuvwattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primuvwattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primuvwattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainoffsetattrib(mut self, val: &str) -> Self {
        self.base.params.insert(
            "terrainoffsetattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terrainoffsetattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "terrainoffsetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectsamples(mut self, val: bool) -> Self {
        self.base.params.insert(
            "projectsamples".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectsamples_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projectsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_projectforces(mut self, val: bool) -> Self {
        self.base.params.insert(
            "projectforces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectforces_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "projectforces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenormalattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usenormalattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenormalattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usenormalattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useprimnumattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useprimnumattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useprimnumattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useprimnumattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useprimuvwattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useprimuvwattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useprimuvwattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useprimuvwattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useterrainoffsetattrib(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useterrainoffsetattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useterrainoffsetattrib_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useterrainoffsetattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAgentterrainprojection {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "agentterrainprojection"
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
pub enum DopAnchoralignaxisParmopObjectaxis {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchoralignaxisParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchoralignaxisCondof {
    FreeToRotateInAnyDirection = 0,
    /// Rotation Axis Lies on Plane (Specify Normal)
    RotationAxisLiesOnPlaneSpecifyNormal = 1,
    /// Rotation Around Axis (Specify Axis)
    RotationAroundAxisSpecifyAxis = 2,
    RotationFixed = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchoralignaxisParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchoralignaxisDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchoralignaxisSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchoralignaxis {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchoralignaxis {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Data to attach Accepts 2 of: Anchor: Object Point Group Position Anchor: Object Point Id Position Anchor: Object Point Number Position Anchor: Object Primitive Position Anchor: Object Space Position Anchor: World Space Position"
    pub fn set_input_data_to_attach_accepts_2_of_anchor_objec(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Data to attach Accepts 2 of: Anchor: Object Point Group Position Anchor: Object Point Id Position Anchor: Object Point Number Position Anchor: Object Primitive Position Anchor: Object Space Position Anchor: World Space Position" and specifies the output index of the target node.
    pub fn set_input_data_to_attach_accepts_2_of_anchor_objec_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Data to attach Accepts 2 of: Anchor: Object Point Group Position Anchor: Object Point Id Position Anchor: Object Point Number Position Anchor: Object Primitive Position Anchor: Object Space Position Anchor: World Space Position"
    pub fn set_input_data_to_attach_accepts_2_of_anchor_objec_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Data to attach Accepts 2 of: Anchor: Object Point Group Position Anchor: Object Point Id Position Anchor: Object Point Number Position Anchor: Object Primitive Position Anchor: Object Space Position Anchor: World Space Position" and specifies the output index of the target node.
    pub fn set_input_data_to_attach_accepts_2_of_anchor_objec_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_objectaxis(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "objectaxis".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "objectaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condof(mut self, val: DopAnchoralignaxisCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_objectaxis(mut self, val: DopAnchoralignaxisParmopObjectaxis) -> Self {
        self.base.params.insert(
            "parmop_objectaxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_objectaxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_objectaxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchoralignaxisParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchoralignaxisParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchoralignaxisDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchoralignaxisSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchoralignaxis {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchoralignaxis"
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
pub enum DopAnchorobjpointgroupposParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopGeometry {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopPointgroup {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopOffsetattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopCondofattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopCondirattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposParmopFusepidattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgroupposSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjpointgrouppos {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjpointgrouppos {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condof(mut self, val: DopAnchorobjpointgroupposCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointgroupposParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointgroupposParmopGeometry) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointgroup(
        mut self,
        val: DopAnchorobjpointgroupposParmopPointgroup,
    ) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjpointgroupposParmopOffset) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_offsetattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopOffsetattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_offsetattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offsetattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_offsetattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointgroupposParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condofattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopCondofattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_condofattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condofattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condofattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointgroupposParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condirattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopCondirattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_condirattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condirattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condirattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_fusepidattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopFusepidattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_fusepidattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_fusepidattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_fusepidattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointgroupposDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointgroupposSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offsetattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "offsetattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_offsetattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "offsetattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condofattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "condofattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_condofattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condofattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condirattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "condirattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_condirattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condirattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fusepidattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "fusepidattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fusepidattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fusepidattribname".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjpointgrouppos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjpointgrouppos"
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
pub enum DopAnchorobjpointgrouprotParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopGeometry {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopPointgroup {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopOrientattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopUpattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopOutattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopR {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopRotationattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopCondofattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotParmopCondirattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointgrouprotSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjpointgrouprot {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjpointgrouprot {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condof(mut self, val: DopAnchorobjpointgrouprotCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointgrouprotParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointgrouprotParmopGeometry) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointgroup(
        mut self,
        val: DopAnchorobjpointgrouprotParmopPointgroup,
    ) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_orientattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopOrientattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_orientattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_orientattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_orientattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_upattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopUpattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_upattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_upattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_upattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_outattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopOutattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_outattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_outattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_outattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjpointgrouprotParmopR) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_rotationattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopRotationattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_rotationattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_rotationattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_rotationattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointgrouprotParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condofattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopCondofattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_condofattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condofattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condofattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointgrouprotParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condirattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopCondirattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_condirattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condirattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condirattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointgrouprotDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointgrouprotSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "orientattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orientattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "upattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotationattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rotationattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rotationattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotationattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condofattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "condofattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_condofattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condofattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condirattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "condirattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_condirattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condirattribname".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjpointgrouprot {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjpointgrouprot"
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
pub enum DopAnchorobjpointidposParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopGeometry {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopUseworldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopPointid {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopIdattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidposSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjpointidpos {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjpointidpos {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pointid(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pointid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointidposCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointidposParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointidposParmopGeometry) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointidposParmopUseworldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointidposParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointid(mut self, val: DopAnchorobjpointidposParmopPointid) -> Self {
        self.base.params.insert(
            "parmop_pointid".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_idattribname(
        mut self,
        val: DopAnchorobjpointidposParmopIdattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_idattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_idattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_idattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjpointidposParmopOffset) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointidposParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointidposParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointidposDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointidposSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "idattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_idattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "idattribname".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjpointidpos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjpointidpos"
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
pub enum DopAnchorobjpointidrotParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopGeometry {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopPointid {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopIdattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopOrientattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopUpattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopOutattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopR {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopUseworldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointidrotSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjpointidrot {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjpointidrot {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pointid(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pointid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointidrotCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointidrotParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointidrotParmopGeometry) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointid(mut self, val: DopAnchorobjpointidrotParmopPointid) -> Self {
        self.base.params.insert(
            "parmop_pointid".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointid_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_idattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopIdattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_idattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_idattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_idattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_orientattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopOrientattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_orientattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_orientattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_orientattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_upattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopUpattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_upattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_upattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_upattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_outattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopOutattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_outattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_outattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_outattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjpointidrotParmopR) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointidrotParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointidrotParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointidrotParmopUseworldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointidrotParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointidrotDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointidrotSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "idattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_idattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "idattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "orientattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orientattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "upattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outattribname".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjpointidrot {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjpointidrot"
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
pub enum DopAnchorobjpointnumposParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposParmopGeometry {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposParmopUseworldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposParmopPointnum {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumposSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjpointnumpos {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjpointnumpos {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pointnum(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pointnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointnumposCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointnumposParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointnumposParmopGeometry) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointnumposParmopUseworldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointnumposParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointnum(mut self, val: DopAnchorobjpointnumposParmopPointnum) -> Self {
        self.base.params.insert(
            "parmop_pointnum".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjpointnumposParmopOffset) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointnumposParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointnumposParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointnumposDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointnumposSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjpointnumpos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjpointnumpos"
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
pub enum DopAnchorobjpointnumrotParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopGeometry {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopPointnum {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopOrientattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopUpattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopOutattribname {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopR {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopUseworldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjpointnumrotSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjpointnumrot {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjpointnumrot {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pointnum(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pointnum".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointnumrotCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointnumrotParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointnumrotParmopGeometry) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointnum(mut self, val: DopAnchorobjpointnumrotParmopPointnum) -> Self {
        self.base.params.insert(
            "parmop_pointnum".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointnum_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_orientattribname(
        mut self,
        val: DopAnchorobjpointnumrotParmopOrientattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_orientattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_orientattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_orientattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_upattribname(
        mut self,
        val: DopAnchorobjpointnumrotParmopUpattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_upattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_upattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_upattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_outattribname(
        mut self,
        val: DopAnchorobjpointnumrotParmopOutattribname,
    ) -> Self {
        self.base.params.insert(
            "parmop_outattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_outattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_outattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjpointnumrotParmopR) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointnumrotParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointnumrotParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointnumrotParmopUseworldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointnumrotParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointnumrotDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointnumrotSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "geometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "orientattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_orientattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "orientattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "upattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outattribname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outattribname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outattribname".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjpointnumrot {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjpointnumrot"
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
pub enum DopAnchorobjprimposParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposParmopUseworldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposParmopPrimitive {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposParmopPrimuv {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjprimposSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjprimpos {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjprimpos {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float2 parameters ---
    pub fn with_primuv(mut self, val: [f32; 2]) -> Self {
        self.base.params.insert(
            "primuv".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_primuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_primitive(mut self, val: i32) -> Self {
        self.base.params.insert(
            "primitive".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primitive_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primitive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjprimposCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjprimposParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjprimposParmopUseworldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjprimposParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_primitive(mut self, val: DopAnchorobjprimposParmopPrimitive) -> Self {
        self.base.params.insert(
            "parmop_primitive".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_primitive_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_primitive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_primuv(mut self, val: DopAnchorobjprimposParmopPrimuv) -> Self {
        self.base.params.insert(
            "parmop_primuv".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_primuv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_primuv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjprimposParmopOffset) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjprimposParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjprimposParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjprimposDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjprimposSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjprimpos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjprimpos"
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
pub enum DopAnchorobjregionParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopPrimitivegroup {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopRegistrationattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopAllowpartialoverlap {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopEnablematching {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopIdentifierattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopEnablemultipliers {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopStrengthattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopDampingattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopEnabletargetpositionattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionParmopTargetpositionattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjregionSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjregion {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjregion {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjregionParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_primitivegroup(
        mut self,
        val: DopAnchorobjregionParmopPrimitivegroup,
    ) -> Self {
        self.base.params.insert(
            "parmop_primitivegroup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_primitivegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_primitivegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_registrationattribute(
        mut self,
        val: DopAnchorobjregionParmopRegistrationattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_registrationattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_registrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_allowpartialoverlap(
        mut self,
        val: DopAnchorobjregionParmopAllowpartialoverlap,
    ) -> Self {
        self.base.params.insert(
            "parmop_allowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_allowpartialoverlap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_allowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablematching(
        mut self,
        val: DopAnchorobjregionParmopEnablematching,
    ) -> Self {
        self.base.params.insert(
            "parmop_enablematching".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablematching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enablematching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_identifierattribute(
        mut self,
        val: DopAnchorobjregionParmopIdentifierattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_identifierattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_identifierattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_identifierattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablemultipliers(
        mut self,
        val: DopAnchorobjregionParmopEnablemultipliers,
    ) -> Self {
        self.base.params.insert(
            "parmop_enablemultipliers".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablemultipliers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enablemultipliers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_strengthattribute(
        mut self,
        val: DopAnchorobjregionParmopStrengthattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_strengthattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_strengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_dampingattribute(
        mut self,
        val: DopAnchorobjregionParmopDampingattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_dampingattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_dampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute(
        mut self,
        val: DopAnchorobjregionParmopEnabletargetpositionattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute(
        mut self,
        val: DopAnchorobjregionParmopTargetpositionattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjregionDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjregionSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primitivegroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "primitivegroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primitivegroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "primitivegroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_registrationattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "registrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "registrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_identifierattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "identifierattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_identifierattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "identifierattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strengthattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "strengthattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dampingattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetpositionattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetpositionattribute".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_allowpartialoverlap(mut self, val: bool) -> Self {
        self.base.params.insert(
            "allowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowpartialoverlap_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "allowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablematching(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablematching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablematching_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablematching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemultipliers(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablemultipliers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemultipliers_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablemultipliers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjregion {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjregion"
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
pub enum DopAnchorobjspaceposParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposParmopUseworldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspaceposSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjspacepos {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjspacepos {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
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
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condof(mut self, val: DopAnchorobjspaceposCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjspaceposParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjspaceposParmopUseworldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjspaceposParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjspaceposParmopOffset) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjspaceposParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjspaceposParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjspaceposDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjspaceposSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjspacepos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjspacepos"
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
pub enum DopAnchorobjspacerotParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotParmopUseworldspacerotation {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotParmopWorldspacerotation {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotParmopR {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotCondof {
    FreeToRotateInAnyDirection = 0,
    /// Rotation Axis Lies on Plane (Specify Normal)
    RotationAxisLiesOnPlaneSpecifyNormal = 1,
    /// Rotation Around Axis (Specify Axis)
    RotationAroundAxisSpecifyAxis = 2,
    RotationFixed = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotParmopUseworldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjspacerotSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjspacerot {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjspacerot {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_worldspacerotation(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacerotation".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacerotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacerotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condof(mut self, val: DopAnchorobjspacerotCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjspacerotParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacerotation(
        mut self,
        val: DopAnchorobjspacerotParmopUseworldspacerotation,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacerotation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacerotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacerotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacerotation(
        mut self,
        val: DopAnchorobjspacerotParmopWorldspacerotation,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacerotation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacerotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacerotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjspacerotParmopR) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjspacerotParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjspacerotParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjspacerotParmopUseworldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjspacerotParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjspacerotDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjspacerotSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useworldspacerotation(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacerotation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacerotation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacerotation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useworldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjspacerot {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjspacerot"
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
pub enum DopAnchorobjsurfaceposParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopRegistrationattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopPointgroup {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopUsedistancethreshold {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopDistancethreshold {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopEnabledistancethresholdattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopDistancethresholdattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopFilterbyborderside {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopBorderside {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposBorderside {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopFilterbyselfocclusion {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopConnectionmodel {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposConnectionmodel {
    /// Two-Sided: Attract And Repel
    TwoMinusSidedAttractAndRepel = 0,
    /// One-Sided: Only Repel
    OneMinusSidedOnlyRepel = 1,
    /// One-Sided: Only Attract
    OneMinusSidedOnlyAttract = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopEnablestrengthattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopStrengthattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopEnabledampingattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopDampingattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopRestinitialization {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposRestinitialization {
    Zero = 0,
    Reference = 1,
    Absolute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopRestdistance {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopEnablerestdistanceattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopRestdistanceattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopAttachtolocalspace {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopEnabletargetrelativeattributes {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopTargetprimitiveattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopTargetsubindexattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopTargetfaceidattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopTargetbaryattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopEnabletargetpositionattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposParmopTargetpositionattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceposSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjsurfacepos {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjsurfacepos {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restdistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjsurfaceposParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_registrationattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopRegistrationattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_registrationattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_registrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointgroup(mut self, val: DopAnchorobjsurfaceposParmopPointgroup) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold(
        mut self,
        val: DopAnchorobjsurfaceposParmopUsedistancethreshold,
    ) -> Self {
        self.base.params.insert(
            "parmop_usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_distancethreshold(
        mut self,
        val: DopAnchorobjsurfaceposParmopDistancethreshold,
    ) -> Self {
        self.base.params.insert(
            "parmop_distancethreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_distancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabledistancethresholdattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopDistancethresholdattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_filterbyborderside(
        mut self,
        val: DopAnchorobjsurfaceposParmopFilterbyborderside,
    ) -> Self {
        self.base.params.insert(
            "parmop_filterbyborderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_filterbyborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_borderside(mut self, val: DopAnchorobjsurfaceposParmopBorderside) -> Self {
        self.base.params.insert(
            "parmop_borderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_borderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_borderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_borderside(mut self, val: DopAnchorobjsurfaceposBorderside) -> Self {
        self.base.params.insert(
            "borderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "borderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion(
        mut self,
        val: DopAnchorobjsurfaceposParmopFilterbyselfocclusion,
    ) -> Self {
        self.base.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_connectionmodel(
        mut self,
        val: DopAnchorobjsurfaceposParmopConnectionmodel,
    ) -> Self {
        self.base.params.insert(
            "parmop_connectionmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_connectionmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_connectionmodel(mut self, val: DopAnchorobjsurfaceposConnectionmodel) -> Self {
        self.base.params.insert(
            "connectionmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "connectionmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnablestrengthattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_strengthattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopStrengthattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_strengthattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_strengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabledampingattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_dampingattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopDampingattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_dampingattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_dampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceposParmopRestinitialization,
    ) -> Self {
        self.base.params.insert(
            "parmop_restinitialization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restinitialization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_restinitialization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceposRestinitialization,
    ) -> Self {
        self.base.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restinitialization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_restdistance(
        mut self,
        val: DopAnchorobjsurfaceposParmopRestdistance,
    ) -> Self {
        self.base.params.insert(
            "parmop_restdistance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_restdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnablerestdistanceattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopRestdistanceattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_attachtolocalspace(
        mut self,
        val: DopAnchorobjsurfaceposParmopAttachtolocalspace,
    ) -> Self {
        self.base.params.insert(
            "parmop_attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_attachtolocalspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabletargetrelativeattributes,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetprimitiveattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetsubindexattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetfaceidattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetbaryattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetbaryattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetbaryattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabletargetpositionattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetpositionattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjsurfaceposDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjsurfaceposSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_registrationattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "registrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "registrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distancethresholdattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strengthattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "strengthattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dampingattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetprimitiveattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsubindexattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetfaceidattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetbaryattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetbaryattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetbaryattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetpositionattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetpositionattribute".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbyborderside(mut self, val: bool) -> Self {
        self.base.params.insert(
            "filterbyborderside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filterbyborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbyselfocclusion(mut self, val: bool) -> Self {
        self.base.params.insert(
            "filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablestrengthattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledampingattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerestdistanceattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attachtolocalspace(mut self, val: bool) -> Self {
        self.base.params.insert(
            "attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attachtolocalspace_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjsurfacepos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjsurfacepos"
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
pub enum DopAnchorobjsurfaceslideParmopObject {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopRegistrationattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopPointgroup {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopUsedistancethreshold {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopDistancethreshold {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopEnabledistancethresholdattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopDistancethresholdattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopFilterbyborderside {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopBorderside {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideBorderside {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopFilterbyselfocclusion {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopConnectionmodel {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideConnectionmodel {
    /// Two-Sided: Attract And Repel
    TwoMinusSidedAttractAndRepel = 0,
    /// One-Sided: Only Repel
    OneMinusSidedOnlyRepel = 1,
    /// One-Sided: Only Attract
    OneMinusSidedOnlyAttract = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopEnablestrengthattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopStrengthattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopEnabledampingattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopDampingattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopRestinitialization {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideRestinitialization {
    Zero = 0,
    Reference = 1,
    Absolute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopRestdistance {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopEnablerestdistanceattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopRestdistanceattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopEnabletargetrelativeattributes {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopTargetprimitiveattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopTargetsubindexattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopTargetfaceidattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopTargetbaryattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopEnabletargetpositionattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideParmopTargetpositionattribute {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorobjsurfaceslideSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorobjsurfaceslide {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorobjsurfaceslide {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restdistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_object(mut self, val: DopAnchorobjsurfaceslideParmopObject) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_registrationattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRegistrationattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_registrationattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_registrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_pointgroup(mut self, val: DopAnchorobjsurfaceslideParmopPointgroup) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold(
        mut self,
        val: DopAnchorobjsurfaceslideParmopUsedistancethreshold,
    ) -> Self {
        self.base.params.insert(
            "parmop_usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_distancethreshold(
        mut self,
        val: DopAnchorobjsurfaceslideParmopDistancethreshold,
    ) -> Self {
        self.base.params.insert(
            "parmop_distancethreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_distancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabledistancethresholdattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopDistancethresholdattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_filterbyborderside(
        mut self,
        val: DopAnchorobjsurfaceslideParmopFilterbyborderside,
    ) -> Self {
        self.base.params.insert(
            "parmop_filterbyborderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_filterbyborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_borderside(mut self, val: DopAnchorobjsurfaceslideParmopBorderside) -> Self {
        self.base.params.insert(
            "parmop_borderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_borderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_borderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_borderside(mut self, val: DopAnchorobjsurfaceslideBorderside) -> Self {
        self.base.params.insert(
            "borderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "borderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion(
        mut self,
        val: DopAnchorobjsurfaceslideParmopFilterbyselfocclusion,
    ) -> Self {
        self.base.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_connectionmodel(
        mut self,
        val: DopAnchorobjsurfaceslideParmopConnectionmodel,
    ) -> Self {
        self.base.params.insert(
            "parmop_connectionmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_connectionmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_connectionmodel(mut self, val: DopAnchorobjsurfaceslideConnectionmodel) -> Self {
        self.base.params.insert(
            "connectionmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "connectionmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnablestrengthattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_strengthattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopStrengthattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_strengthattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_strengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabledampingattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_dampingattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopDampingattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_dampingattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_dampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRestinitialization,
    ) -> Self {
        self.base.params.insert(
            "parmop_restinitialization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restinitialization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_restinitialization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceslideRestinitialization,
    ) -> Self {
        self.base.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restinitialization_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_restdistance(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRestdistance,
    ) -> Self {
        self.base.params.insert(
            "parmop_restdistance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_restdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnablerestdistanceattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRestdistanceattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabletargetrelativeattributes,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetprimitiveattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetsubindexattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetfaceidattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetbaryattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetbaryattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetbaryattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabletargetpositionattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetpositionattribute,
    ) -> Self {
        self.base.params.insert(
            "parmop_targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjsurfaceslideDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjsurfaceslideSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object(mut self, val: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "object".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_registrationattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "registrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "registrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pointgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distancethresholdattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strengthattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "strengthattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "strengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dampingattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetprimitiveattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsubindexattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetfaceidattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetbaryattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetbaryattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetbaryattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "targetpositionattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "targetpositionattribute".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbyborderside(mut self, val: bool) -> Self {
        self.base.params.insert(
            "filterbyborderside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filterbyborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbyselfocclusion(mut self, val: bool) -> Self {
        self.base.params.insert(
            "filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "filterbyselfocclusion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablestrengthattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledampingattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerestdistanceattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletargetrelativeattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorobjsurfaceslide {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorobjsurfaceslide"
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
pub enum DopAnchortargetSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchortarget {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchortarget {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopAnchortargetSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchortarget {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchortarget"
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
pub enum DopAnchorworldspaceposParmopT {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspaceposParmopVel {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspaceposParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspaceposCondof {
    FreeToMoveInAnyDirection = 0,
    /// Constrained to Plane (Specify Normal)
    ConstrainedToPlaneSpecifyNormal = 1,
    /// Constrained to Line (Specify Direction)
    ConstrainedToLineSpecifyDirection = 2,
    ConstrainedToPoint = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspaceposParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspaceposDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspaceposSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorworldspacepos {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorworldspacepos {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condof(mut self, val: DopAnchorworldspaceposCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_t(mut self, val: DopAnchorworldspaceposParmopT) -> Self {
        self.base.params.insert(
            "parmop_t".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_vel(mut self, val: DopAnchorworldspaceposParmopVel) -> Self {
        self.base.params.insert(
            "parmop_vel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_vel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorworldspaceposParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorworldspaceposParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorworldspaceposDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorworldspaceposSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorworldspacepos {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorworldspacepos"
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
pub enum DopAnchorworldspacerotParmopR {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspacerotParmopAngvel {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspacerotParmopCondof {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspacerotCondof {
    FreeToRotateInAnyDirection = 0,
    /// Rotation Axis Lies on Plane (Specify Normal)
    RotationAxisLiesOnPlaneSpecifyNormal = 1,
    /// Rotation Around Axis (Specify Axis)
    RotationAroundAxisSpecifyAxis = 2,
    RotationFixed = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspacerotParmopCondir {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspacerotParmopWorldspacepos {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspacerotDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchorworldspacerotSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchorworldspacerot {
    pub base: crate::core::types::NodeBase,
}

impl DopAnchorworldspacerot {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float3 parameters ---
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condof(mut self, val: DopAnchorworldspacerotCondof) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_r(mut self, val: DopAnchorworldspacerotParmopR) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_angvel(mut self, val: DopAnchorworldspacerotParmopAngvel) -> Self {
        self.base.params.insert(
            "parmop_angvel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_angvel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorworldspacerotParmopCondof) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condof".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorworldspacerotParmopCondir) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_condir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorworldspacerotParmopWorldspacepos,
    ) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parmop_worldspacepos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorworldspacerotDefaultparmop) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorworldspacerotSharedata) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopAnchorworldspacerot {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "anchorworldspacerot"
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
pub struct DopApplydata {
    pub base: crate::core::types::NodeBase,
}

impl DopApplydata {
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
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
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

impl crate::core::types::HoudiniNode for DopApplydata {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "applydata"
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
pub enum DopApplyrelCreaterels {
    NumberOfRelationships = 0,
    NumberOfRelationshipsPerAffectedObject = 1,
    NumberOfRelationshipsPerAffectorObject = 2,
    NumberOfRelationshipsPerObjectPair = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopApplyrelVareval {
    EvaluateForEachCopy = 0,
    EvaluateOnce = 1,
    /// Evaluate Once, One Token per Copy
    EvaluateOnceOneTokenPerCopy = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopApplyrelVartype {
    Float = 0,
    String = 1,
}

#[derive(Debug, Clone)]
pub struct DopApplyrel {
    pub base: crate::core::types::NodeBase,
}

impl DopApplyrel {
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

    // --- Float parameters ---
    pub fn with_varfloat_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("varfloat{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_varfloat_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("varfloat{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_numrels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "numrels".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numrels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numrels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createrels(mut self, val: DopApplyrelCreaterels) -> Self {
        self.base.params.insert(
            "createrels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createrels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createrels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vareval_inst(mut self, index1: usize, val: DopApplyrelVareval) -> Self {
        self.base.params.insert(
            format!("vareval{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vareval_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vareval{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vartype_inst(mut self, index1: usize, val: DopApplyrelVartype) -> Self {
        self.base.params.insert(
            format!("vartype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vartype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("vartype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_affected(mut self, val: &str) -> Self {
        self.base.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affected_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affectors(mut self, val: &str) -> Self {
        self.base.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affectors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("varname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_varname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("varname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varstring_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("varstring{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_varstring_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("varstring{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquerelname(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquerelname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_makemutual(mut self, val: bool) -> Self {
        self.base.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makemutual_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopApplyrel {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "applyrel"
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
