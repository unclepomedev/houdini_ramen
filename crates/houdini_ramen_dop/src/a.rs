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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopActivevalue {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_active(mut self, val: f32) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_active_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_active(mut self, val: DopActivevalueParmopActive) -> Self {
        self.params.insert(
            "parmop_active".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_active_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_active".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopActivevalueDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopActivevalueSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopActivevalue {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "activevalue"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopActivevalueOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopActivevalueOutputs for DopActivevalue {}
impl DopActivevalueOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopActivevalue> {}

#[derive(Debug, Clone)]
pub struct DopAffector {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAffector {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affector_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("affector{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affector_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("affector{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affected_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("affected{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affected_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("affected{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAffector {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "affector"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAffectorOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAffectorOutputs for DopAffector {}
impl DopAffectorOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopAffector> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentarcingcliplayerTurndirection {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Clone)]
pub struct DopAgentarcingcliplayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAgentarcingcliplayer {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_stream_to_apply_clips_to_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_turnradius_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("turnradius_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_turnradius_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("turnradius_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_turndirection_inst(
        mut self,
        index1: usize,
        val: DopAgentarcingcliplayerTurndirection,
    ) -> Self {
        self.params.insert(
            format!("turndirection_{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_turndirection_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("turndirection_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("clipname_{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipname_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAgentarcingcliplayer {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "agentarcingcliplayer"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAgentarcingcliplayerOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
}

impl DopAgentarcingcliplayerOutputs for DopAgentarcingcliplayer {}
impl DopAgentarcingcliplayerOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAgentarcingcliplayer>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAgentcliplayer {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_agents_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_trigger_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_trigger_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_trigger_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input4<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input4".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_randomclipseed_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("randomclipseed_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipseed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("randomclipseed_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clippatternweight_inst(mut self, index1: usize, index2: usize, val: f32) -> Self {
        self.params.insert(
            format!("clippatternweight_{}_{}", index1, index2),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clippatternweight_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("clippatternweight_{}_{}", index1, index2),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blendratio_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("blendratio_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendratio_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("blendratio_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("weight_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("weight_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initialcliptime_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("initialcliptime_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_initialcliptime_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("initialcliptime_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomclipoffset_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("randomclipoffset_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipoffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("randomclipoffset_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomclipoffsetseed_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("randomclipoffsetseed_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomclipoffsetseed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("randomclipoffsetseed_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipspeedmultiplier_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("clipspeedmultiplier_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedmultiplier_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipspeedmultiplier_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_locomotionspeedmultiplier_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("locomotionspeedmultiplier_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_locomotionspeedmultiplier_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("locomotionspeedmultiplier_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipspeedvariance_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("clipspeedvariance_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedvariance_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipspeedvariance_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipspeedseed_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("clipspeedseed_{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipspeedseed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipspeedseed_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blendinframes_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("blendinframes_{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blendinframes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("blendinframes_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blendoutframes_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("blendoutframes_{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blendoutframes_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("blendoutframes_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blendmode(mut self, val: DopAgentcliplayerBlendmode) -> Self {
        self.params.insert(
            "blendmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blendmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_triggerinput_inst(
        mut self,
        index1: usize,
        val: DopAgentcliplayerTriggerinput,
    ) -> Self {
        self.params.insert(
            format!("triggerinput_{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_triggerinput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("triggerinput_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("clipname_{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipname_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clippattern_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("clippattern_{}_{}", index1, index2),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clippattern_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clippattern_{}_{}", index1, index2),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_transformgroup_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("transformgroup_{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_transformgroup_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("transformgroup_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localexpression_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("localexpression_{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localexpression_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("localexpression_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localweightexpression(mut self, val: &str) -> Self {
        self.params.insert(
            "localweightexpression".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localweightexpression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localweightexpression".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_cwdpath(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_cwdpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_cwdpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomizeclips_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("randomizeclips_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizeclips_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("randomizeclips_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetrigger_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usetrigger_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetrigger_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usetrigger_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_triggeronce_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("triggeronce_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_triggeronce_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("triggeronce_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setinitialcliptime_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("setinitialcliptime_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setinitialcliptime_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("setinitialcliptime_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomizecliptime_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("randomizecliptime_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizecliptime_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("randomizecliptime_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overrideclipspeed_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("overrideclipspeed_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideclipspeed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("overrideclipspeed_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomizeclipspeed_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("randomizeclipspeed_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizeclipspeed_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("randomizeclipspeed_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uselocal_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("uselocal_{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocal_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("uselocal_{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uselocalweight(mut self, val: bool) -> Self {
        self.params.insert(
            "uselocalweight".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocalweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselocalweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAgentcliplayer {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "agentcliplayer"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAgentcliplayerOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
}

impl DopAgentcliplayerOutputs for DopAgentcliplayer {}
impl DopAgentcliplayerOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAgentcliplayer>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAgentlookat {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_agents_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_hozlimitangle(mut self, val: f32) -> Self {
        self.params.insert(
            "hozlimitangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hozlimitangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hozlimitangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vertlimitangle(mut self, val: f32) -> Self {
        self.params.insert(
            "vertlimitangle".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vertlimitangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vertlimitangle".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distanceweight(mut self, val: f32) -> Self {
        self.params.insert(
            "distanceweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distanceweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distanceweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_peripheryweight(mut self, val: f32) -> Self {
        self.params.insert(
            "peripheryweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_peripheryweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "peripheryweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relorientweight(mut self, val: f32) -> Self {
        self.params.insert(
            "relorientweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_relorientweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relorientweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relspeedweight(mut self, val: f32) -> Self {
        self.params.insert(
            "relspeedweight".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_relspeedweight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relspeedweight".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scoremultiplier(mut self, val: f32) -> Self {
        self.params.insert(
            "scoremultiplier".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scoremultiplier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scoremultiplier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomdurationoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "randomdurationoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomdurationoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomdurationoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomdurationseed(mut self, val: f32) -> Self {
        self.params.insert(
            "randomdurationseed".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_randomdurationseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomdurationseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "distancerange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_distancerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancerange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relspeedrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "relspeedrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_relspeedrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relspeedrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_durationtime(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "durationtime".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_durationtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "durationtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_position(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "position".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_position_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "position".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activate(mut self, val: i32) -> Self {
        self.params.insert(
            "activate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxsearchpts(mut self, val: i32) -> Self {
        self.params.insert(
            "maxsearchpts".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsearchpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxsearchpts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxneighbors(mut self, val: i32) -> Self {
        self.params.insert(
            "maxneighbors".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxneighbors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxneighbors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targettype(mut self, val: DopAgentlookatTargettype) -> Self {
        self.params.insert(
            "targettype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targettype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targettype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometrysource(mut self, val: DopAgentlookatGeometrysource) -> Self {
        self.params.insert(
            "geometrysource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geometrysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometrysource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hozlimitmode(mut self, val: DopAgentlookatHozlimitmode) -> Self {
        self.params.insert(
            "hozlimitmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hozlimitmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hozlimitmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vertlimitmode(mut self, val: DopAgentlookatVertlimitmode) -> Self {
        self.params.insert(
            "vertlimitmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vertlimitmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vertlimitmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_durationtimemode(mut self, val: DopAgentlookatDurationtimemode) -> Self {
        self.params.insert(
            "durationtimemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_durationtimemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "durationtimemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetname(mut self, val: &str) -> Self {
        self.params.insert(
            "targetname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpath(mut self, val: &str) -> Self {
        self.params.insert(
            "objectpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dopobject(mut self, val: &str) -> Self {
        self.params.insert(
            "dopobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dopobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dopdataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dopdataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dopdataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopdataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_searchgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "searchgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_searchgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "searchgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_neighborgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "neighborgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_neighborgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "neighborgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_agentattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "agentattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_agentattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "agentattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "targetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hozlimitattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "hozlimitattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hozlimitattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hozlimitattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vertlimitattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "vertlimitattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vertlimitattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vertlimitattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_durationtimeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "durationtimeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_durationtimeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "durationtimeattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matchbyattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "matchbyattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchbyattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchbyattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetargetattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "usetargetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetargetattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetargetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomizedurationtime(mut self, val: bool) -> Self {
        self.params.insert(
            "randomizedurationtime".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomizedurationtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomizedurationtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAgentlookat {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "agentlookat"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAgentlookatOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
}

impl DopAgentlookatOutputs for DopAgentlookat {}
impl DopAgentlookatOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopAgentlookat> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAgentlookatapplyMintargetscoremode {
    SetValue = 0,
    SetFromAttribute = 1,
    ScaleByAttribute = 2,
}

#[derive(Debug, Clone)]
pub struct DopAgentlookatapply {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAgentlookatapply {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_agents_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_activate(mut self, val: f32) -> Self {
        self.params.insert(
            "activate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mintargetscore(mut self, val: f32) -> Self {
        self.params.insert(
            "mintargetscore".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mintargetscore_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintargetscore".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_headturnstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "headturnstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headturnstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headturnstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_headweightscale(mut self, val: f32) -> Self {
        self.params.insert(
            "headweightscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_headweightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headweightscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeturnstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "eyeturnstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeturnstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeturnstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeweightscale(mut self, val: f32) -> Self {
        self.params.insert(
            "eyeweightscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_eyeweightscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeweightscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_headweightanglerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "headweightanglerange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_headweightanglerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headweightanglerange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeweightanglerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "eyeweightanglerange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_eyeweightanglerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeweightanglerange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_refdir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refdir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_refup(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refup".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numjoints(mut self, val: i32) -> Self {
        self.params.insert(
            "numjoints".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numjoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numjoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mintargetscoremode(mut self, val: DopAgentlookatapplyMintargetscoremode) -> Self {
        self.params.insert(
            "mintargetscoremode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mintargetscoremode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintargetscoremode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_headweightramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "headweightramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_headweightramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "headweightramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_eyeweightramp(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "eyeweightramp".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_eyeweightramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "eyeweightramp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mintargetscoreattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "mintargetscoreattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mintargetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintargetscoreattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetnameattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "targetnameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetnameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetnameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetposattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "targetposattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetposattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetposattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetscoreattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "targetscoreattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetscoreattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setnumjoints(mut self, val: bool) -> Self {
        self.params.insert(
            "setnumjoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setnumjoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setnumjoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_immediateadjust(mut self, val: bool) -> Self {
        self.params.insert(
            "immediateadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_immediateadjust_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "immediateadjust".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtargetnameattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtargetnameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetnameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtargetnameattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtargetposattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtargetposattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetposattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtargetposattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtargetscoreattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "outputtargetscoreattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputtargetscoreattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtargetscoreattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAgentlookatapply {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "agentlookatapply"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAgentlookatapplyOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
}

impl DopAgentlookatapplyOutputs for DopAgentlookatapply {}
impl DopAgentlookatapplyOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAgentlookatapply>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAgentterrainadaptation {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_agents_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_activate(mut self, val: f32) -> Self {
        self.params.insert(
            "activate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "hipoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipoffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hipshiftperframe(mut self, val: f32) -> Self {
        self.params.insert(
            "hipshiftperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hipshiftperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipshiftperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_kneedampingthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "kneedampingthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_kneedampingthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "kneedampingthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tiltangleperframe(mut self, val: f32) -> Self {
        self.params.insert(
            "tiltangleperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tiltangleperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tiltangleperframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mintilt(mut self, val: f32) -> Self {
        self.params.insert(
            "mintilt".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mintilt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mintilt".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxtilt(mut self, val: f32) -> Self {
        self.params.insert(
            "maxtilt".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtilt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxtilt".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "guidescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lockedscale(mut self, val: f32) -> Self {
        self.params.insert(
            "lockedscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lockedscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lockedscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_refdir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refdir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_refup(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "refup".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_refup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "refup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainsource(mut self, val: DopAgentterrainadaptationTerrainsource) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_terrainsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainobject(mut self, val: &str) -> Self {
        self.params.insert(
            "terrainobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainsoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "terrainsoppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainsoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainsoppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terraingroup(mut self, val: &str) -> Self {
        self.params.insert(
            "terraingroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terraingroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terraingroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablefootlocking(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefootlocking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefootlocking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefootlocking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adjusthips(mut self, val: bool) -> Self {
        self.params.insert(
            "adjusthips".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adjusthips_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adjusthips".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limithipshift(mut self, val: bool) -> Self {
        self.params.insert(
            "limithipshift".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limithipshift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limithipshift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablekneedamping(mut self, val: bool) -> Self {
        self.params.insert(
            "enablekneedamping".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablekneedamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablekneedamping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableterrainadaptation(mut self, val: bool) -> Self {
        self.params.insert(
            "enableterrainadaptation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableterrainadaptation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableterrainadaptation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableleaning(mut self, val: bool) -> Self {
        self.params.insert(
            "enableleaning".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableleaning_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableleaning".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAgentterrainadaptation {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "agentterrainadaptation"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAgentterrainadaptationOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Sub-Network Output #1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
}

impl DopAgentterrainadaptationOutputs for DopAgentterrainadaptation {}
impl DopAgentterrainadaptationOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAgentterrainadaptation>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAgentterrainprojection {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_agents_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projectiondir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "projectiondir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_projectiondir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectiondir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enable(mut self, val: i32) -> Self {
        self.params.insert(
            "enable".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainsource(mut self, val: DopAgentterrainprojectionTerrainsource) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_terrainsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mode(mut self, val: DopAgentterrainprojectionMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_samplingmethod(mut self, val: DopAgentterrainprojectionSamplingmethod) -> Self {
        self.params.insert(
            "samplingmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplingmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplingmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_partgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_binddopinput2(mut self, val: &str) -> Self {
        self.params.insert(
            "binddopinput2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_binddopinput2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "binddopinput2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindgeoinput2(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeoinput2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindgeoinput2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeoinput2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terraingroup(mut self, val: &str) -> Self {
        self.params.insert(
            "terraingroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terraingroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terraingroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normalattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "normalattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_normalattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primnumattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "primnumattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primnumattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primnumattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primuvwattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "primuvwattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primuvwattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primuvwattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainoffsetattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "terrainoffsetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_terrainoffsetattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainoffsetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usegroup(mut self, val: bool) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projectsamples(mut self, val: bool) -> Self {
        self.params.insert(
            "projectsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectsamples".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_projectforces(mut self, val: bool) -> Self {
        self.params.insert(
            "projectforces".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_projectforces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "projectforces".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usenormalattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "usenormalattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenormalattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenormalattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useprimnumattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "useprimnumattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useprimnumattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useprimnumattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useprimuvwattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "useprimuvwattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useprimuvwattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useprimuvwattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useterrainoffsetattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "useterrainoffsetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useterrainoffsetattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useterrainoffsetattrib".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAgentterrainprojection {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "agentterrainprojection"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAgentterrainprojectionOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
}

impl DopAgentterrainprojectionOutputs for DopAgentterrainprojection {}
impl DopAgentterrainprojectionOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAgentterrainprojection>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchoralignaxis {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_data_to_attach_accepts_2_of_anchor_objec_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_data_to_attach_accepts_2_of_anchor_objec_1_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_objectaxis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchoralignaxisCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_objectaxis(mut self, val: DopAnchoralignaxisParmopObjectaxis) -> Self {
        self.params.insert(
            "parmop_objectaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_objectaxis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_objectaxis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchoralignaxisParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchoralignaxisParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchoralignaxisDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchoralignaxisSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchoralignaxis {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchoralignaxis"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchoralignaxisOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchoralignaxisOutputs for DopAnchoralignaxis {}
impl DopAnchoralignaxisOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchoralignaxis>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjpointgrouppos {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointgroupposCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointgroupposParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointgroupposParmopGeometry) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointgroup(
        mut self,
        val: DopAnchorobjpointgroupposParmopPointgroup,
    ) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjpointgroupposParmopOffset) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_offsetattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopOffsetattribname,
    ) -> Self {
        self.params.insert(
            "parmop_offsetattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offsetattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_offsetattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointgroupposParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condofattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopCondofattribname,
    ) -> Self {
        self.params.insert(
            "parmop_condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condofattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointgroupposParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condirattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopCondirattribname,
    ) -> Self {
        self.params.insert(
            "parmop_condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condirattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_fusepidattribname(
        mut self,
        val: DopAnchorobjpointgroupposParmopFusepidattribname,
    ) -> Self {
        self.params.insert(
            "parmop_fusepidattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_fusepidattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_fusepidattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointgroupposDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointgroupposSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offsetattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "offsetattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offsetattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offsetattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condofattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condofattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condirattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condirattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fusepidattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "fusepidattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fusepidattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fusepidattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjpointgrouppos {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjpointgrouppos"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjpointgroupposOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjpointgroupposOutputs for DopAnchorobjpointgrouppos {}
impl DopAnchorobjpointgroupposOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjpointgrouppos>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjpointgrouprot {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointgrouprotCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointgrouprotParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointgrouprotParmopGeometry) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointgroup(
        mut self,
        val: DopAnchorobjpointgrouprotParmopPointgroup,
    ) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_orientattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopOrientattribname,
    ) -> Self {
        self.params.insert(
            "parmop_orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_orientattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_upattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopUpattribname,
    ) -> Self {
        self.params.insert(
            "parmop_upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_upattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_outattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopOutattribname,
    ) -> Self {
        self.params.insert(
            "parmop_outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_outattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjpointgrouprotParmopR) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_rotationattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopRotationattribname,
    ) -> Self {
        self.params.insert(
            "parmop_rotationattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_rotationattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_rotationattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointgrouprotParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condofattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopCondofattribname,
    ) -> Self {
        self.params.insert(
            "parmop_condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condofattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointgrouprotParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condirattribname(
        mut self,
        val: DopAnchorobjpointgrouprotParmopCondirattribname,
    ) -> Self {
        self.params.insert(
            "parmop_condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condirattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointgrouprotDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointgrouprotSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rotationattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "rotationattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rotationattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rotationattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condofattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condofattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condofattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condirattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condirattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condirattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjpointgrouprot {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjpointgrouprot"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjpointgrouprotOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjpointgrouprotOutputs for DopAnchorobjpointgrouprot {}
impl DopAnchorobjpointgrouprotOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjpointgrouprot>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjpointidpos {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointid(mut self, val: i32) -> Self {
        self.params.insert(
            "pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointidposCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointidposParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointidposParmopGeometry) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointidposParmopUseworldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointidposParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointid(mut self, val: DopAnchorobjpointidposParmopPointid) -> Self {
        self.params.insert(
            "parmop_pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_idattribname(
        mut self,
        val: DopAnchorobjpointidposParmopIdattribname,
    ) -> Self {
        self.params.insert(
            "parmop_idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_idattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjpointidposParmopOffset) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointidposParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointidposParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointidposDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointidposSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjpointidpos {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjpointidpos"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjpointidposOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjpointidposOutputs for DopAnchorobjpointidpos {}
impl DopAnchorobjpointidposOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjpointidpos>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjpointidrot {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointid(mut self, val: i32) -> Self {
        self.params.insert(
            "pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointidrotCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointidrotParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointidrotParmopGeometry) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointid(mut self, val: DopAnchorobjpointidrotParmopPointid) -> Self {
        self.params.insert(
            "parmop_pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointid".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_idattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopIdattribname,
    ) -> Self {
        self.params.insert(
            "parmop_idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_idattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_orientattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopOrientattribname,
    ) -> Self {
        self.params.insert(
            "parmop_orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_orientattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_upattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopUpattribname,
    ) -> Self {
        self.params.insert(
            "parmop_upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_upattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_outattribname(
        mut self,
        val: DopAnchorobjpointidrotParmopOutattribname,
    ) -> Self {
        self.params.insert(
            "parmop_outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_outattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjpointidrotParmopR) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointidrotParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointidrotParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointidrotParmopUseworldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointidrotParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointidrotDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointidrotSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_idattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjpointidrot {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjpointidrot"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjpointidrotOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjpointidrotOutputs for DopAnchorobjpointidrot {}
impl DopAnchorobjpointidrotOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjpointidrot>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjpointnumpos {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointnum(mut self, val: i32) -> Self {
        self.params.insert(
            "pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointnumposCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointnumposParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointnumposParmopGeometry) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointnumposParmopUseworldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointnumposParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointnum(mut self, val: DopAnchorobjpointnumposParmopPointnum) -> Self {
        self.params.insert(
            "parmop_pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjpointnumposParmopOffset) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointnumposParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointnumposParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointnumposDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointnumposSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjpointnumpos {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjpointnumpos"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjpointnumposOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjpointnumposOutputs for DopAnchorobjpointnumpos {}
impl DopAnchorobjpointnumposOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjpointnumpos>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjpointnumrot {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointnum(mut self, val: i32) -> Self {
        self.params.insert(
            "pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pointnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjpointnumrotCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjpointnumrotParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_geometry(mut self, val: DopAnchorobjpointnumrotParmopGeometry) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointnum(mut self, val: DopAnchorobjpointnumrotParmopPointnum) -> Self {
        self.params.insert(
            "parmop_pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointnum".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_orientattribname(
        mut self,
        val: DopAnchorobjpointnumrotParmopOrientattribname,
    ) -> Self {
        self.params.insert(
            "parmop_orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_orientattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_upattribname(
        mut self,
        val: DopAnchorobjpointnumrotParmopUpattribname,
    ) -> Self {
        self.params.insert(
            "parmop_upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_upattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_outattribname(
        mut self,
        val: DopAnchorobjpointnumrotParmopOutattribname,
    ) -> Self {
        self.params.insert(
            "parmop_outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_outattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjpointnumrotParmopR) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjpointnumrotParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjpointnumrotParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjpointnumrotParmopUseworldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjpointnumrotParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjpointnumrotDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjpointnumrotSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry(mut self, val: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_orientattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjpointnumrot {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjpointnumrot"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjpointnumrotOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjpointnumrotOutputs for DopAnchorobjpointnumrot {}
impl DopAnchorobjpointnumrotOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjpointnumrot>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjprimpos {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_primuv(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "primuv".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_primuv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primuv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primitive(mut self, val: i32) -> Self {
        self.params.insert(
            "primitive".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_primitive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitive".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjprimposCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjprimposParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjprimposParmopUseworldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjprimposParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_primitive(mut self, val: DopAnchorobjprimposParmopPrimitive) -> Self {
        self.params.insert(
            "parmop_primitive".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_primitive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_primitive".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_primuv(mut self, val: DopAnchorobjprimposParmopPrimuv) -> Self {
        self.params.insert(
            "parmop_primuv".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_primuv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_primuv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjprimposParmopOffset) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjprimposParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjprimposParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjprimposDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjprimposSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjprimpos {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjprimpos"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjprimposOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjprimposOutputs for DopAnchorobjprimpos {}
impl DopAnchorobjprimposOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjprimpos>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjregion {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjregionParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_primitivegroup(
        mut self,
        val: DopAnchorobjregionParmopPrimitivegroup,
    ) -> Self {
        self.params.insert(
            "parmop_primitivegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_primitivegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_primitivegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_registrationattribute(
        mut self,
        val: DopAnchorobjregionParmopRegistrationattribute,
    ) -> Self {
        self.params.insert(
            "parmop_registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_allowpartialoverlap(
        mut self,
        val: DopAnchorobjregionParmopAllowpartialoverlap,
    ) -> Self {
        self.params.insert(
            "parmop_allowpartialoverlap".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_allowpartialoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_allowpartialoverlap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enablematching(
        mut self,
        val: DopAnchorobjregionParmopEnablematching,
    ) -> Self {
        self.params.insert(
            "parmop_enablematching".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablematching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablematching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_identifierattribute(
        mut self,
        val: DopAnchorobjregionParmopIdentifierattribute,
    ) -> Self {
        self.params.insert(
            "parmop_identifierattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_identifierattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_identifierattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enablemultipliers(
        mut self,
        val: DopAnchorobjregionParmopEnablemultipliers,
    ) -> Self {
        self.params.insert(
            "parmop_enablemultipliers".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablemultipliers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablemultipliers".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_strengthattribute(
        mut self,
        val: DopAnchorobjregionParmopStrengthattribute,
    ) -> Self {
        self.params.insert(
            "parmop_strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_dampingattribute(
        mut self,
        val: DopAnchorobjregionParmopDampingattribute,
    ) -> Self {
        self.params.insert(
            "parmop_dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute(
        mut self,
        val: DopAnchorobjregionParmopEnabletargetpositionattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute(
        mut self,
        val: DopAnchorobjregionParmopTargetpositionattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjregionDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjregionSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primitivegroup(mut self, val: &str) -> Self {
        self.params.insert(
            "primitivegroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primitivegroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitivegroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_registrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_identifierattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "identifierattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_identifierattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "identifierattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strengthattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dampingattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_allowpartialoverlap(mut self, val: bool) -> Self {
        self.params.insert(
            "allowpartialoverlap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowpartialoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowpartialoverlap".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablematching(mut self, val: bool) -> Self {
        self.params.insert(
            "enablematching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablematching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablematching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablemultipliers(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemultipliers".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemultipliers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemultipliers".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjregion {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjregion"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjregionOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjregionOutputs for DopAnchorobjregion {}
impl DopAnchorobjregionOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjregion>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjspacepos {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjspaceposCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjspaceposParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjspaceposParmopUseworldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjspaceposParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_offset(mut self, val: DopAnchorobjspaceposParmopOffset) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjspaceposParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjspaceposParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjspaceposDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjspaceposSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjspacepos {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjspacepos"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjspaceposOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjspaceposOutputs for DopAnchorobjspacepos {}
impl DopAnchorobjspaceposOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjspacepos>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjspacerot {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_worldspacerotation(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacerotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorobjspacerotCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjspacerotParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacerotation(
        mut self,
        val: DopAnchorobjspacerotParmopUseworldspacerotation,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacerotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacerotation(
        mut self,
        val: DopAnchorobjspacerotParmopWorldspacerotation,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacerotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorobjspacerotParmopR) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorobjspacerotParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorobjspacerotParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_useworldspacepos(
        mut self,
        val: DopAnchorobjspacerotParmopUseworldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorobjspacerotParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjspacerotDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjspacerotSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacerotation(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacerotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacerotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useworldspacepos(mut self, val: bool) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useworldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useworldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjspacerot {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjspacerot"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjspacerotOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjspacerotOutputs for DopAnchorobjspacerot {}
impl DopAnchorobjspacerotOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjspacerot>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjsurfacepos {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjsurfaceposParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_registrationattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopRegistrationattribute,
    ) -> Self {
        self.params.insert(
            "parmop_registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointgroup(mut self, val: DopAnchorobjsurfaceposParmopPointgroup) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold(
        mut self,
        val: DopAnchorobjsurfaceposParmopUsedistancethreshold,
    ) -> Self {
        self.params.insert(
            "parmop_usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancethreshold(
        mut self,
        val: DopAnchorobjsurfaceposParmopDistancethreshold,
    ) -> Self {
        self.params.insert(
            "parmop_distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabledistancethresholdattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopDistancethresholdattribute,
    ) -> Self {
        self.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_filterbyborderside(
        mut self,
        val: DopAnchorobjsurfaceposParmopFilterbyborderside,
    ) -> Self {
        self.params.insert(
            "parmop_filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_borderside(mut self, val: DopAnchorobjsurfaceposParmopBorderside) -> Self {
        self.params.insert(
            "parmop_borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_borderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_borderside(mut self, val: DopAnchorobjsurfaceposBorderside) -> Self {
        self.params.insert(
            "borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion(
        mut self,
        val: DopAnchorobjsurfaceposParmopFilterbyselfocclusion,
    ) -> Self {
        self.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_connectionmodel(
        mut self,
        val: DopAnchorobjsurfaceposParmopConnectionmodel,
    ) -> Self {
        self.params.insert(
            "parmop_connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_connectionmodel(mut self, val: DopAnchorobjsurfaceposConnectionmodel) -> Self {
        self.params.insert(
            "connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnablestrengthattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_strengthattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopStrengthattribute,
    ) -> Self {
        self.params.insert(
            "parmop_strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabledampingattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_dampingattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopDampingattribute,
    ) -> Self {
        self.params.insert(
            "parmop_dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceposParmopRestinitialization,
    ) -> Self {
        self.params.insert(
            "parmop_restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restinitialization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceposRestinitialization,
    ) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restinitialization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_restdistance(
        mut self,
        val: DopAnchorobjsurfaceposParmopRestdistance,
    ) -> Self {
        self.params.insert(
            "parmop_restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnablerestdistanceattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopRestdistanceattribute,
    ) -> Self {
        self.params.insert(
            "parmop_restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_attachtolocalspace(
        mut self,
        val: DopAnchorobjsurfaceposParmopAttachtolocalspace,
    ) -> Self {
        self.params.insert(
            "parmop_attachtolocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_attachtolocalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_attachtolocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabletargetrelativeattributes,
    ) -> Self {
        self.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetprimitiveattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetsubindexattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetfaceidattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetbaryattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopEnabletargetpositionattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceposParmopTargetpositionattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjsurfaceposDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjsurfaceposSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_registrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancethresholdattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strengthattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dampingattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdistanceattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetprimitiveattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetsubindexattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetfaceidattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetbaryattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filterbyborderside(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filterbyselfocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablestrengthattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabledampingattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablerestdistanceattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attachtolocalspace(mut self, val: bool) -> Self {
        self.params.insert(
            "attachtolocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attachtolocalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attachtolocalspace".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjsurfacepos {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjsurfacepos"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjsurfaceposOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjsurfaceposOutputs for DopAnchorobjsurfacepos {}
impl DopAnchorobjsurfaceposOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjsurfacepos>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorobjsurfaceslide {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_object(mut self, val: DopAnchorobjsurfaceslideParmopObject) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_registrationattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRegistrationattribute,
    ) -> Self {
        self.params.insert(
            "parmop_registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_pointgroup(mut self, val: DopAnchorobjsurfaceslideParmopPointgroup) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold(
        mut self,
        val: DopAnchorobjsurfaceslideParmopUsedistancethreshold,
    ) -> Self {
        self.params.insert(
            "parmop_usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancethreshold(
        mut self,
        val: DopAnchorobjsurfaceslideParmopDistancethreshold,
    ) -> Self {
        self.params.insert(
            "parmop_distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabledistancethresholdattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopDistancethresholdattribute,
    ) -> Self {
        self.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_filterbyborderside(
        mut self,
        val: DopAnchorobjsurfaceslideParmopFilterbyborderside,
    ) -> Self {
        self.params.insert(
            "parmop_filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_borderside(mut self, val: DopAnchorobjsurfaceslideParmopBorderside) -> Self {
        self.params.insert(
            "parmop_borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_borderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_borderside(mut self, val: DopAnchorobjsurfaceslideBorderside) -> Self {
        self.params.insert(
            "borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_borderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "borderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion(
        mut self,
        val: DopAnchorobjsurfaceslideParmopFilterbyselfocclusion,
    ) -> Self {
        self.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_connectionmodel(
        mut self,
        val: DopAnchorobjsurfaceslideParmopConnectionmodel,
    ) -> Self {
        self.params.insert(
            "parmop_connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_connectionmodel(mut self, val: DopAnchorobjsurfaceslideConnectionmodel) -> Self {
        self.params.insert(
            "connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectionmodel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnablestrengthattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_strengthattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopStrengthattribute,
    ) -> Self {
        self.params.insert(
            "parmop_strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabledampingattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_dampingattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopDampingattribute,
    ) -> Self {
        self.params.insert(
            "parmop_dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRestinitialization,
    ) -> Self {
        self.params.insert(
            "parmop_restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restinitialization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restinitialization(
        mut self,
        val: DopAnchorobjsurfaceslideRestinitialization,
    ) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restinitialization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_restdistance(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRestdistance,
    ) -> Self {
        self.params.insert(
            "parmop_restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restdistance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnablerestdistanceattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopRestdistanceattribute,
    ) -> Self {
        self.params.insert(
            "parmop_restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabletargetrelativeattributes,
    ) -> Self {
        self.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetprimitiveattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetsubindexattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetfaceidattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetbaryattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopEnabletargetpositionattribute,
    ) -> Self {
        self.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute(
        mut self,
        val: DopAnchorobjsurfaceslideParmopTargetpositionattribute,
    ) -> Self {
        self.params.insert(
            "parmop_targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorobjsurfaceslideDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorobjsurfaceslideSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object(mut self, val: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_registrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_registrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "registrationattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pointgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pointgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancethresholdattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strengthattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_strengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dampingattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdistanceattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetprimitiveattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetprimitiveattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetsubindexattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetsubindexattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetfaceidattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetfaceidattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetbaryattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetbaryattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledistancethresholdattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filterbyborderside(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbyborderside".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filterbyselfocclusion(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbyselfocclusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbyselfocclusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablestrengthattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabledampingattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablerestdistanceattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetrelativeattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetrelativeattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorobjsurfaceslide {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorobjsurfaceslide"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorobjsurfaceslideOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorobjsurfaceslideOutputs for DopAnchorobjsurfaceslide {}
impl DopAnchorobjsurfaceslideOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorobjsurfaceslide>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopAnchortargetSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopAnchortarget {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchortarget {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchortargetSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchortarget {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchortarget"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchortargetOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchortargetOutputs for DopAnchortarget {}
impl DopAnchortargetOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchortarget> {}

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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorworldspacepos {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorworldspaceposCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_t(mut self, val: DopAnchorworldspaceposParmopT) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_vel(mut self, val: DopAnchorworldspaceposParmopVel) -> Self {
        self.params.insert(
            "parmop_vel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorworldspaceposParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorworldspaceposParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorworldspaceposDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorworldspaceposSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorworldspacepos {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorworldspacepos"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorworldspaceposOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorworldspaceposOutputs for DopAnchorworldspacepos {}
impl DopAnchorworldspaceposOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorworldspacepos>
{
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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopAnchorworldspacerot {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_worldspacepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condof(mut self, val: DopAnchorworldspacerotCondof) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_r(mut self, val: DopAnchorworldspacerotParmopR) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_angvel(mut self, val: DopAnchorworldspacerotParmopAngvel) -> Self {
        self.params.insert(
            "parmop_angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condof(mut self, val: DopAnchorworldspacerotParmopCondof) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condof_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condof".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_condir(mut self, val: DopAnchorworldspacerotParmopCondir) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_worldspacepos(
        mut self,
        val: DopAnchorworldspacerotParmopWorldspacepos,
    ) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_worldspacepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_worldspacepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopAnchorworldspacerotDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopAnchorworldspacerotSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopAnchorworldspacerot {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "anchorworldspacerot"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopAnchorworldspacerotOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopAnchorworldspacerotOutputs for DopAnchorworldspacerot {}
impl DopAnchorworldspacerotOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopAnchorworldspacerot>
{
}

#[derive(Debug, Clone)]
pub struct DopApplydata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DopApplydata {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn add_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(self.next_input_index),
            (out.node_id, out.pin),
        );
        self.next_input_index += 1;
        self
    }

    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopApplydata {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "applydata"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopApplydataOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopApplydataOutputs for DopApplydata {}
impl DopApplydataOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopApplydata> {}

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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DopApplyrel {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
        }
    }

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn add_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(self.next_input_index),
            (out.node_id, out.pin),
        );
        self.next_input_index += 1;
        self
    }

    pub fn with_varfloat_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("varfloat{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_varfloat_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varfloat{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numrels(mut self, val: i32) -> Self {
        self.params.insert(
            "numrels".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numrels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numrels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createrels(mut self, val: DopApplyrelCreaterels) -> Self {
        self.params.insert(
            "createrels".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createrels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createrels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vareval_inst(mut self, index1: usize, val: DopApplyrelVareval) -> Self {
        self.params.insert(
            format!("vareval{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vareval_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vareval{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vartype_inst(mut self, index1: usize, val: DopApplyrelVartype) -> Self {
        self.params.insert(
            format!("vartype{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vartype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vartype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affected(mut self, val: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affectors(mut self, val: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("varname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varstring_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("varstring{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_varstring_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varstring{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquerelname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquerelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_makemutual(mut self, val: bool) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makemutual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopApplyrel {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "applyrel"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait DopApplyrelOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopApplyrelOutputs for DopApplyrel {}
impl DopApplyrelOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopApplyrel> {}
