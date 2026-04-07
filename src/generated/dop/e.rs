#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopEmbeddingpropertiesParmopEnableembedding {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopEmbeddingpropertiesDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopEmbeddingpropertiesSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopEmbeddingproperties {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopEmbeddingproperties {
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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_enableembedding(
        mut self,
        val: DopEmbeddingpropertiesParmopEnableembedding,
    ) -> Self {
        self.params.insert(
            "parmop_enableembedding".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enableembedding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enableembedding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopEmbeddingpropertiesDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopEmbeddingpropertiesSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enableembedding(mut self, val: bool) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableembedding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopEmbeddingproperties {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "embeddingproperties"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopEmptydataSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopEmptydata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopEmptydata {
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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopEmptydataSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopEmptydata {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "emptydata"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub struct DopEmptyobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopEmptyobject {
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

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objname(mut self, val: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowcaching(mut self, val: bool) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowcaching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopEmptyobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "emptyobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopEmptyrelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopEmptyrel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopEmptyrel {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopEmptyrelSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affected(mut self, val: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_affectors(mut self, val: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_affectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquerelname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquerelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_makemutual(mut self, val: bool) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makemutual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopEmptyrel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "emptyrel"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopEnablesolverEnabledatamode {
    AllDataPresent = 0,
    AnyDataPresent = 1,
    AllDataAbsent = 2,
    AnyDataAbsent = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopEnablesolverEnableattribmode {
    AllAttributesPresent = 0,
    AnyAttributePresent = 1,
    AllAttributesAbsent = 2,
    AnyAttributeAbsent = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopEnablesolverEnableattribclass {
    Detail = 0,
    Primitive = 1,
    Point = 2,
    Vertex = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopEnablesolverEnablefieldmode {
    All = 0,
    Any = 1,
    AnyNot = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopEnablesolverEnablefieldtest {
    Constant = 0,
    Constant0 = 1,
    ConstantBorder = 2,
}

#[derive(Debug, Clone)]
pub struct DopEnablesolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopEnablesolver {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_enable(mut self, val: i32) -> Self {
        self.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_enabledatamode(mut self, val: DopEnablesolverEnabledatamode) -> Self {
        self.params.insert(
            "enabledatamode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_enabledatamode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledatamode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableattribmode(mut self, val: DopEnablesolverEnableattribmode) -> Self {
        self.params.insert(
            "enableattribmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_enableattribmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableattribmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableattribclass(mut self, val: DopEnablesolverEnableattribclass) -> Self {
        self.params.insert(
            "enableattribclass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_enableattribclass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableattribclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefieldmode(mut self, val: DopEnablesolverEnablefieldmode) -> Self {
        self.params.insert(
            "enablefieldmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_enablefieldmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefieldmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefieldtest(mut self, val: DopEnablesolverEnablefieldtest) -> Self {
        self.params.insert(
            "enablefieldtest".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_enablefieldtest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefieldtest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_enableobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "enableobjects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_enableobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledata(mut self, val: &str) -> Self {
        self.params.insert(
            "enabledata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_enabledata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "enableattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_enableattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefield(mut self, val: &str) -> Self {
        self.params.insert(
            "enablefield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_enablefield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solverperobject(mut self, val: bool) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solverperobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopEnablesolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "enablesolver"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
