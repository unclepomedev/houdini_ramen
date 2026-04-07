#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWaitforallPdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWaitforallPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWaitforallPdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWaitforallPdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWaitforallPdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWaitforallPdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWaitforallPdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopWaitforall {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopWaitforall {
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

    // --- Menu parameters ---
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopWaitforallPdgPartitionmergeoutputs,
    ) -> Self {
        self.params.insert(
            "pdg_partitionmergeoutputs".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_partitionmergeoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionmergeoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergeoperation_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("pdg_partitionmergeoperation{}", index1),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_partitionmergeoperation_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergeoperation{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionwhen(mut self, val: TopWaitforallPdgPartitionwhen) -> Self {
        self.params.insert(
            "pdg_partitionwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_partitionwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitiontargettype(
        mut self,
        val: TopWaitforallPdgPartitiontargettype,
    ) -> Self {
        self.params.insert(
            "pdg_partitiontargettype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_partitiontargettype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitiontargettype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplitmissing(
        mut self,
        val: TopWaitforallPdgPartitionsplitmissing,
    ) -> Self {
        self.params.insert(
            "pdg_partitionsplitmissing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_partitionsplitmissing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsplitmissing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionframe(mut self, val: TopWaitforallPdgPartitionframe) -> Self {
        self.params.insert(
            "pdg_partitionframe".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_partitionframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsort(mut self, val: TopWaitforallPdgPartitionsort) -> Self {
        self.params.insert(
            "pdg_partitionsort".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_partitionsort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsortdirection(
        mut self,
        val: TopWaitforallPdgPartitionsortdirection,
    ) -> Self {
        self.params.insert(
            "pdg_partitionsortdirection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_partitionsortdirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsortdirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_partitionmergepattern_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepattern{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergepattern_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepattern{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitiontarget(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_partitiontarget".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_partitiontarget_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitiontarget".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplitattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_partitionsplitattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplitattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsplitattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplitdefault(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_partitionsplitdefault".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplitdefault_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsplitdefault".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsortattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_partitionsortattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsortattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsortattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionidattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_partitionidattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionidattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionidattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_pdg_partitionmerge(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_partitionmerge".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionmerge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionmerge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergepreserve_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepreserve{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionmergepreserve_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdg_partitionmergepreserve{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionignorefailures(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_partitionignorefailures".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionignorefailures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionignorefailures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplit(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_partitionsplit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionsplit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsplit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplitpartial(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_partitionsplitpartial".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionsplitpartial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsplitpartial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionstoreids(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_partitionstoreids".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionstoreids_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionstoreids".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopWaitforall {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "waitforall"
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
pub enum TopWedgeWorkitemorder {
    GroupByInput = 0,
    GroupByWedge = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWedgeCapturetype {
    Float = 0,
    FloatVector = 1,
    Integer = 2,
    IntegerVector = 3,
    String = 4,
    Color = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWedgeType {
    Float = 0,
    FloatVector = 1,
    Integer = 2,
    IntegerVector = 3,
    String = 4,
    Color = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWedgeValuetype {
    AttributeReference = 0,
    ParameterValue = 1,
    ParameterExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWedgeWedgetype {
    Range = 0,
    Value = 1,
    ValueList = 2,
    Bracket = 3,
}

#[derive(Debug, Clone)]
pub struct TopWedge {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopWedge {
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

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Button parameters ---
    pub fn trigger_captureall(mut self) -> Self {
        self.params.insert(
            "captureall".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_restoreall(mut self) -> Self {
        self.params.insert(
            "restoreall".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_channeljump_inst(mut self, index1: usize) -> Self {
        self.params.insert(
            format!("channeljump{}", index1),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_channelpicker_inst(mut self, index1: usize) -> Self {
        self.params.insert(
            format!("channelpicker{}", index1),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_capturenumeric_inst(mut self, index1: usize) -> Self {
        self.params.insert(
            format!("capturenumeric{}", index1),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_restorenumeric_inst(mut self, index1: usize) -> Self {
        self.params.insert(
            format!("restorenumeric{}", index1),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_capturestring_inst(mut self, index1: usize) -> Self {
        self.params.insert(
            format!("capturestring{}", index1),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_restorestring_inst(mut self, index1: usize) -> Self {
        self.params.insert(
            format!("restorestring{}", index1),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_floatvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("floatvalue{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_floatvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatvalue_inst_1(mut self, index1: usize, index2: usize, val: f32) -> Self {
        self.params.insert(
            format!("floatvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_floatvalue_inst_1_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_floatrange_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("floatrange{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_floatrange_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatrange{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatbracket_inst(mut self, index1: usize, val: [f32; 2]) -> Self {
        self.params.insert(
            format!("floatbracket{}", index1),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_floatbracket_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatbracket{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_capturednumeric_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("capturednumeric{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_capturednumeric_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("capturednumeric{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatvectorvalue_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatvectorvalue{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_floatvectorvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvectorvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorvalue_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("colorvalue{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_colorvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colorvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatrangestart_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatrangestart{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_floatrangestart_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatrangestart{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatrangeend_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatrangeend{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_floatrangeend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatrangeend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorrangestart_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("colorrangestart{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_colorrangestart_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colorrangestart{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorrangeend_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("colorrangeend{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_colorrangeend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colorrangeend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatvectorcenter_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatvectorcenter{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_floatvectorcenter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvectorcenter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatvectoroffset_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatvectoroffset{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_floatvectoroffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvectoroffset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorcenter_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("colorcenter{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_colorcenter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colorcenter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coloroffset_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("coloroffset{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_coloroffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("coloroffset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatvector_inst(mut self, index1: usize, index2: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatvector{}_{}", index1, index2),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_floatvector_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvector{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_colorvalue_inst_1(mut self, index1: usize, index2: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("colorvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_colorvalue_inst_1_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("colorvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_wedgecount(mut self, val: i32) -> Self {
        self.params.insert(
            "wedgecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_wedgecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wedgecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params
            .insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("intvalue{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_intvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intvalue_inst_1(mut self, index1: usize, index2: usize, val: i32) -> Self {
        self.params.insert(
            format!("intvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_intvalue_inst_1_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_intrange_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.params.insert(
            format!("intrange{}", index1),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_intrange_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intrange{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intbracket_inst(mut self, index1: usize, val: [i32; 2]) -> Self {
        self.params.insert(
            format!("intbracket{}", index1),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_intbracket_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intbracket{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int4 parameters ---
    pub fn with_intvectorvalue_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intvectorvalue{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_intvectorvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvectorvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intrangestart_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intrangestart{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_intrangestart_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intrangestart{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intrangeend_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intrangeend{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_intrangeend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intrangeend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intvectorcenter_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intvectorcenter{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_intvectorcenter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvectorcenter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intvectoroffset_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intvectoroffset{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_intvectoroffset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvectoroffset{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intvector_inst(mut self, index1: usize, index2: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intvector{}_{}", index1, index2),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_intvector_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvector{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_workitemorder(mut self, val: TopWedgeWorkitemorder) -> Self {
        self.params.insert(
            "workitemorder".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_workitemorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "workitemorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capturetype_inst(mut self, index1: usize, val: TopWedgeCapturetype) -> Self {
        self.params.insert(
            format!("capturetype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_capturetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("capturetype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: TopWedgeType) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuetype_inst(mut self, index1: usize, val: TopWedgeValuetype) -> Self {
        self.params.insert(
            format!("valuetype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_valuetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("valuetype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wedgetype_inst(mut self, index1: usize, val: TopWedgeWedgetype) -> Self {
        self.params.insert(
            format!("wedgetype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wedgetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("wedgetype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_channel_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("channel{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_channel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("channel{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_capturedstring_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("capturedstring{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_capturedstring_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("capturedstring{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strvalue_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("strvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_strvalue_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("strvalue{}_{}", index1, index2),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wedgecountattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "wedgecountattrname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_wedgecountattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wedgecountattrname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wedgeindexattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "wedgeindexattrname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_wedgeindexattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wedgeindexattrname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wedgenumattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "wedgenumattrname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_wedgenumattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wedgenumattrname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wedgetotalattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "wedgetotalattrname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_wedgetotalattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wedgetotalattrname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_preservenum(mut self, val: bool) -> Self {
        self.params.insert(
            "preservenum".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preservenum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preservenum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportenvironment(mut self, val: bool) -> Self {
        self.params.insert(
            "exportenvironment".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportenvironment_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportenvironment".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewselection(mut self, val: bool) -> Self {
        self.params.insert(
            "previewselection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_previewselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportchannel_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("exportchannel{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exportchannel_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("exportchannel{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_runcallbacks_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("runcallbacks{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_runcallbacks_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("runcallbacks{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_random_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("random{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_random_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("random{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createwedgecount(mut self, val: bool) -> Self {
        self.params.insert(
            "createwedgecount".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createwedgecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createwedgecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createwedgeindex(mut self, val: bool) -> Self {
        self.params.insert(
            "createwedgeindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createwedgeindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createwedgeindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createwedgenum(mut self, val: bool) -> Self {
        self.params.insert(
            "createwedgenum".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createwedgenum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createwedgenum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createwedgetotal(mut self, val: bool) -> Self {
        self.params.insert(
            "createwedgetotal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createwedgetotal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createwedgetotal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopWedge {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "wedge"
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
pub enum TopWorkitemexpandExpand {
    UpstreamOutputFiles = 0,
    ItemsInUpstreamPartition = 1,
    UpstreamAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWorkitemexpandExpansiontype {
    OneToOne = 0,
    FixedSize = 1,
    FixedCount = 2,
    FirstN = 3,
    LastN = 4,
}

#[derive(Debug, Clone)]
pub struct TopWorkitemexpand {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopWorkitemexpand {
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

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Float2 parameters ---
    pub fn with_framestart(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "framestart".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_framestart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framestart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_value(mut self, val: i32) -> Self {
        self.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_value_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "value".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expand(mut self, val: TopWorkitemexpandExpand) -> Self {
        self.params.insert(
            "expand".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_expand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expansiontype(mut self, val: TopWorkitemexpandExpansiontype) -> Self {
        self.params.insert(
            "expansiontype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_expansiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expansiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_expandattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "expandattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expandattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expandvalue(mut self, val: &str) -> Self {
        self.params.insert(
            "expandvalue".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expandvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_expandworkitems(mut self, val: bool) -> Self {
        self.params.insert(
            "expandworkitems".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandworkitems_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandworkitems".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_includeexpected(mut self, val: bool) -> Self {
        self.params.insert(
            "includeexpected".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_includeexpected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includeexpected".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveindex(mut self, val: bool) -> Self {
        self.params.insert(
            "preserveindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preserveindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setframe(mut self, val: bool) -> Self {
        self.params.insert(
            "setframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopWorkitemexpand {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "workitemexpand"
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
pub enum TopWorkitemimportPdgWorkitemgeneration {
    EachUpstreamItemIsGenerated = 0,
    AllUpstreamItemsAreGenerated = 1,
    Automatic = 2,
    AllUpstreamItemsAreCooked = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWorkitemimportWorkitemsource {
    TopNode = 0,
    /// Upstream Output File(s)
    UpstreamOutputFileS = 1,
    UpstreamWorkItemIdAttribute = 2,
    CustomFilePath = 3,
    UpstreamDictionaryAttribute = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWorkitemimportUsetime {
    WorkItemFrame = 0,
    CustomTime = 1,
    NetworkEvaluationTime = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWorkitemimportTopdirty {
    None = 0,
    WorkItems = 1,
    WorkItemsAndOutputFiles = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWorkitemimportImportcollision {
    KeepExistingAttribute = 0,
    KeepImportedAttribute = 1,
    ReportWarning = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWorkitemimportPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopWorkitemimportPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopWorkitemimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopWorkitemimport {
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

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_input_by_name<N: crate::core::types::HoudiniNode>(
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

    // --- Float parameters ---
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

    // --- Int parameters ---
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(
        mut self,
        val: TopWorkitemimportPdgWorkitemgeneration,
    ) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_workitemsource(mut self, val: TopWorkitemimportWorkitemsource) -> Self {
        self.params.insert(
            "workitemsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_workitemsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "workitemsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetime(mut self, val: TopWorkitemimportUsetime) -> Self {
        self.params.insert(
            "usetime".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usetime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topdirty(mut self, val: TopWorkitemimportTopdirty) -> Self {
        self.params.insert(
            "topdirty".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_topdirty_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topdirty".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importcollision(mut self, val: TopWorkitemimportImportcollision) -> Self {
        self.params.insert(
            "importcollision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_importcollision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopWorkitemimportPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopWorkitemimportPdgWorkitempriority) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_toppath(mut self, val: &str) -> Self {
        self.params.insert(
            "toppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filetag(mut self, val: &str) -> Self {
        self.params.insert(
            "filetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_idattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "idattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_idattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "idattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "dictattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dictattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dictattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepath(mut self, val: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_topgenerate(mut self, val: bool) -> Self {
        self.params.insert(
            "topgenerate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_topgenerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topgenerate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "topattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_topattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importdatatype(mut self, val: bool) -> Self {
        self.params.insert(
            "importdatatype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importdatatype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importdatatype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "importattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopWorkitemimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "workitemimport"
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
