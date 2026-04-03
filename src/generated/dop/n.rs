#[derive(Debug, Clone)]
pub struct DopNetfetchdata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopNetfetchdata {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }


    // --- Int parameters ---
    pub fn with_port(mut self, val: i32) -> Self {
        self.params.insert("port".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_port_expr(mut self, expr: &str) -> Self {
        self.params.insert("port".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_peer(mut self, val: i32) -> Self {
        self.params.insert("peer".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_peer_expr(mut self, expr: &str) -> Self {
        self.params.insert("peer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_npeer(mut self, val: i32) -> Self {
        self.params.insert("npeer".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_npeer_expr(mut self, expr: &str) -> Self {
        self.params.insert("npeer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srcpeer(mut self, val: i32) -> Self {
        self.params.insert("srcpeer".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_srcpeer_expr(mut self, expr: &str) -> Self {
        self.params.insert("srcpeer".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_address(mut self, val: &str) -> Self {
        self.params.insert("address".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_address_expr(mut self, expr: &str) -> Self {
        self.params.insert("address".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jobname(mut self, val: &str) -> Self {
        self.params.insert("jobname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_jobname_expr(mut self, expr: &str) -> Self {
        self.params.insert("jobname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srcobject(mut self, val: &str) -> Self {
        self.params.insert("srcobject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_srcobject_expr(mut self, expr: &str) -> Self {
        self.params.insert("srcobject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srcdataname(mut self, val: &str) -> Self {
        self.params.insert("srcdataname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_srcdataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("srcdataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DopNetfetchdata {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "netfetchdata"
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
pub enum DopNocolliderSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopNocollider {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopNocollider {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
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
    pub fn with_sharedata(mut self, val: DopNocolliderSharedata) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DopNocollider {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "nocollider"
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
pub enum DopNoconrelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopNoconrel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopNoconrel {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_rad(mut self, val: f32) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_sharedata(mut self, val: DopNoconrelSharedata) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert("showobjectlink".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert("showobjectlink".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DopNoconrel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "noconrel"
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
pub enum DopNoiseParmopSeed {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopFractaldepth {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopRoughness {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopAttenuation {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopFrequency {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopMinvalue {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopMaxvalue {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopOffset {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopNoisetype {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseNoisetype {
    HermiteInterpolation = 0,
    SparseConvolution = 1,
    ImprovedHermite = 2,
    AlligatorNoise = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseParmopScalarnoise {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopNoiseSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopNoise {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopNoise {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_roughness(mut self, val: f32) -> Self {
        self.params.insert("roughness".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert("roughness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attenuation(mut self, val: f32) -> Self {
        self.params.insert("attenuation".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_attenuation_expr(mut self, expr: &str) -> Self {
        self.params.insert("attenuation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_frequency(mut self, val: [f32; 3]) -> Self {
        self.params.insert("frequency".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_frequency_expr(mut self, expr: &str) -> Self {
        self.params.insert("frequency".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_minvalue(mut self, val: [f32; 3]) -> Self {
        self.params.insert("minvalue".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_minvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert("minvalue".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxvalue(mut self, val: [f32; 3]) -> Self {
        self.params.insert("maxvalue".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_maxvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxvalue".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fractaldepth(mut self, val: i32) -> Self {
        self.params.insert("fractaldepth".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_fractaldepth_expr(mut self, expr: &str) -> Self {
        self.params.insert("fractaldepth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_seed(mut self, val: DopNoiseParmopSeed) -> Self {
        self.params.insert("parmop_seed".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_fractaldepth(mut self, val: DopNoiseParmopFractaldepth) -> Self {
        self.params.insert("parmop_fractaldepth".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_fractaldepth_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_fractaldepth".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_roughness(mut self, val: DopNoiseParmopRoughness) -> Self {
        self.params.insert("parmop_roughness".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_roughness_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_roughness".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_attenuation(mut self, val: DopNoiseParmopAttenuation) -> Self {
        self.params.insert("parmop_attenuation".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_attenuation_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_attenuation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_frequency(mut self, val: DopNoiseParmopFrequency) -> Self {
        self.params.insert("parmop_frequency".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_frequency_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_frequency".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_minvalue(mut self, val: DopNoiseParmopMinvalue) -> Self {
        self.params.insert("parmop_minvalue".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_minvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_minvalue".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_maxvalue(mut self, val: DopNoiseParmopMaxvalue) -> Self {
        self.params.insert("parmop_maxvalue".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_maxvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_maxvalue".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_offset(mut self, val: DopNoiseParmopOffset) -> Self {
        self.params.insert("parmop_offset".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_offset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_noisetype(mut self, val: DopNoiseParmopNoisetype) -> Self {
        self.params.insert("parmop_noisetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_noisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_noisetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_noisetype(mut self, val: DopNoiseNoisetype) -> Self {
        self.params.insert("noisetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_noisetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("noisetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_scalarnoise(mut self, val: DopNoiseParmopScalarnoise) -> Self {
        self.params.insert("parmop_scalarnoise".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_scalarnoise_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_scalarnoise".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_defaultparmop(mut self, val: DopNoiseDefaultparmop) -> Self {
        self.params.insert("defaultparmop".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert("defaultparmop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sharedata(mut self, val: DopNoiseSharedata) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_scalarnoise(mut self, val: bool) -> Self {
        self.params.insert("scalarnoise".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_scalarnoise_expr(mut self, expr: &str) -> Self {
        self.params.insert("scalarnoise".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DopNoise {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "noise"
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
pub struct DopNull {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopNull {
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

}

impl crate::core::types::HoudiniNode for DopNull {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "null"
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
