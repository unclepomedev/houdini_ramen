#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributeMode {
    AttributeName = 0,
    SpecificAttributeValues = 1,
    DistinctAttributeValues = 2,
    AttributePattern = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributeSortdirection {
    Ascending = 0,
    Descending = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributeType {
    Integer = 0,
    Float = 1,
    String = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributePdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributePdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributePdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributePdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributePdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributePdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyattributePdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbyattribute {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbyattribute {
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

    // --- Float4 parameters ---
    pub fn with_floatvector_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatvector{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_floatvector_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvector{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_index_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("index{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("index{}", index1),
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

    // --- Int4 parameters ---
    pub fn with_intvector_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intvector{}", index1),
            crate::core::types::ParamValue::Int4(val),
        );
        self
    }
    pub fn with_intvector_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvector{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: TopPartitionbyattributeMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sortdirection(mut self, val: TopPartitionbyattributeSortdirection) -> Self {
        self.params.insert(
            "sortdirection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sortdirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sortdirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: TopPartitionbyattributeType) -> Self {
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
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbyattributePdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbyattributePdgPartitionwhen) -> Self {
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
        val: TopPartitionbyattributePdgPartitiontargettype,
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
        val: TopPartitionbyattributePdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(
        mut self,
        val: TopPartitionbyattributePdgPartitionframe,
    ) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbyattributePdgPartitionsort) -> Self {
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
        val: TopPartitionbyattributePdgPartitionsortdirection,
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
    pub fn with_pattern(mut self, val: &str) -> Self {
        self.params.insert(
            "pattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pattern".to_string(),
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
    pub fn with_stringvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stringvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stringvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stringvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_independent(mut self, val: bool) -> Self {
        self.params.insert(
            "independent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_independent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "independent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createunmatched(mut self, val: bool) -> Self {
        self.params.insert(
            "createunmatched".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createunmatched_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createunmatched".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enableindex{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enableindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for TopPartitionbyattribute {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbyattribute"
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
pub enum TopPartitionbyboundsBoundstype {
    BoundingBox = 0,
    BoundingSphere = 1,
    UpstreamGeometry = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyboundsBoundsintersect {
    BoundingBox = 0,
    BoundingSphere = 1,
    BoundingObject = 2,
    BoundingVolume = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyboundsBoundselement {
    Primitives = 0,
    Points = 1,
    Edges = 2,
    Vertices = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyboundsBoundsGeometrysource {
    SopNode = 0,
    UpstreamOutputFile = 1,
    UpstreamGeometryData = 2,
    CustomFilePath = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyboundsSrcGeometrysource {
    SopNode = 0,
    UpstreamOutputFile = 1,
    UpstreamGeometryData = 2,
    CustomFilePath = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyboundsSrcPreprocess {
    UseGeometryAsIs = 0,
    ConvertToBounds = 1,
    ConvertToPolygons = 2,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbybounds {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPartitionbybounds {
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

    /// Connects to input 0: "Bounding Input"
    pub fn set_input_bounding_input<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Bounding Input" and specifies the output index of the target node.
    pub fn set_input_bounding_input_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_bounding_input_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "Source Input"
    pub fn set_input_source_input<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Source Input" and specifies the output index of the target node.
    pub fn set_input_source_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_source_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_iso(mut self, val: f32) -> Self {
        self.params.insert(
            "iso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_iso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_time(mut self, val: f32) -> Self {
        self.params.insert(
            "bounds_time".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounds_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_time(mut self, val: f32) -> Self {
        self.params.insert(
            "src_time".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_src_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_time".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_boundstype(mut self, val: TopPartitionbyboundsBoundstype) -> Self {
        self.params.insert(
            "boundstype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundstype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundstype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundsintersect(mut self, val: TopPartitionbyboundsBoundsintersect) -> Self {
        self.params.insert(
            "boundsintersect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundsintersect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundsintersect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundselement(mut self, val: TopPartitionbyboundsBoundselement) -> Self {
        self.params.insert(
            "boundselement".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_boundselement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundselement".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_geometrysource(
        mut self,
        val: TopPartitionbyboundsBoundsGeometrysource,
    ) -> Self {
        self.params.insert(
            "bounds_geometrysource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bounds_geometrysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_geometrysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_geometrysource(mut self, val: TopPartitionbyboundsSrcGeometrysource) -> Self {
        self.params.insert(
            "src_geometrysource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_src_geometrysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_geometrysource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_preprocess(mut self, val: TopPartitionbyboundsSrcPreprocess) -> Self {
        self.params.insert(
            "src_preprocess".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_src_preprocess_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_preprocess".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_bounds_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "bounds_soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounds_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_filetag(mut self, val: &str) -> Self {
        self.params.insert(
            "bounds_filetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounds_filetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_filetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_filepath(mut self, val: &str) -> Self {
        self.params.insert(
            "bounds_filepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounds_filepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_filepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_upstreamattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "bounds_upstreamattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounds_upstreamattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_upstreamattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "src_soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_src_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_filetag(mut self, val: &str) -> Self {
        self.params.insert(
            "src_filetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_src_filetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_filetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_filepath(mut self, val: &str) -> Self {
        self.params.insert(
            "src_filepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_src_filepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_filepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_upstreamattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "src_upstreamattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_src_upstreamattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_upstreamattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blockpath(mut self, val: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_blockpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blockpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_inputname(mut self, val: &str) -> Self {
        self.params.insert(
            "bounds_inputname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bounds_inputname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_inputname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_inputname(mut self, val: &str) -> Self {
        self.params.insert(
            "src_inputname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_src_inputname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_inputname".to_string(),
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
    pub fn with_partitionsecondary(mut self, val: bool) -> Self {
        self.params.insert(
            "partitionsecondary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_partitionsecondary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionsecondary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orientedbbox(mut self, val: bool) -> Self {
        self.params.insert(
            "orientedbbox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_orientedbbox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orientedbbox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_invertbounds(mut self, val: bool) -> Self {
        self.params.insert(
            "invertbounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invertbounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invertbounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_includenotwhollycontained(mut self, val: bool) -> Self {
        self.params.insert(
            "includenotwhollycontained".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_includenotwhollycontained_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includenotwhollycontained".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_usetime(mut self, val: bool) -> Self {
        self.params.insert(
            "bounds_usetime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounds_usetime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_usetime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_evaluateattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "bounds_evaluateattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounds_evaluateattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_evaluateattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_asyncload(mut self, val: bool) -> Self {
        self.params.insert(
            "bounds_asyncload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounds_asyncload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_asyncload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounds_mergeinput(mut self, val: bool) -> Self {
        self.params.insert(
            "bounds_mergeinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bounds_mergeinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounds_mergeinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_usetime(mut self, val: bool) -> Self {
        self.params.insert(
            "src_usetime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_src_usetime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_usetime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_evaluateattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "src_evaluateattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_src_evaluateattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_evaluateattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_asyncload(mut self, val: bool) -> Self {
        self.params.insert(
            "src_asyncload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_src_asyncload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_asyncload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_src_mergeinput(mut self, val: bool) -> Self {
        self.params.insert(
            "src_mergeinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_src_mergeinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "src_mergeinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridedefault(mut self, val: bool) -> Self {
        self.params.insert(
            "overridedefault".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridedefault_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridedefault".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopPartitionbybounds {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbybounds"
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
pub enum TopPartitionbycombinationPdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycombinationPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycombinationPdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycombinationPdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycombinationPdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycombinationPdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycombinationPdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbycombination {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbycombination {
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

    // --- Float parameters ---
    pub fn with_keeppercentage(mut self, val: f32) -> Self {
        self.params.insert(
            "keeppercentage".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_keeppercentage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppercentage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sortkey(mut self, val: i32) -> Self {
        self.params.insert(
            "sortkey".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sortkey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sortkey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_combinationsize(mut self, val: i32) -> Self {
        self.params.insert(
            "combinationsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_combinationsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "combinationsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepseed(mut self, val: i32) -> Self {
        self.params.insert(
            "keepseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_keepseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbycombinationPdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(
        mut self,
        val: TopPartitionbycombinationPdgPartitionwhen,
    ) -> Self {
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
        val: TopPartitionbycombinationPdgPartitiontargettype,
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
        val: TopPartitionbycombinationPdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(
        mut self,
        val: TopPartitionbycombinationPdgPartitionframe,
    ) -> Self {
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
    pub fn with_pdg_partitionsort(
        mut self,
        val: TopPartitionbycombinationPdgPartitionsort,
    ) -> Self {
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
        val: TopPartitionbycombinationPdgPartitionsortdirection,
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
    pub fn with_usesortkey(mut self, val: bool) -> Self {
        self.params.insert(
            "usesortkey".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesortkey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesortkey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usekeeppercentage(mut self, val: bool) -> Self {
        self.params.insert(
            "usekeeppercentage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usekeeppercentage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usekeeppercentage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for TopPartitionbycombination {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbycombination"
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
pub enum TopPartitionbycomparisonPdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycomparisonPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycomparisonPdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycomparisonPdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycomparisonPdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycomparisonPdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbycomparisonPdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbycomparison {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPartitionbycomparison {
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

    /// Connects to input 0: "primaryinput"
    pub fn set_input_primaryinput<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "primaryinput" and specifies the output index of the target node.
    pub fn set_input_primaryinput_from<N: crate::core::types::HoudiniNode>(
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

    pub fn set_input_primaryinput_by_name<N: crate::core::types::HoudiniNode>(
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

    /// Connects to input 1: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
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
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: "comparison"
    pub fn set_input_comparison<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "comparison" and specifies the output index of the target node.
    pub fn set_input_comparison_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_comparison_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbycomparisonPdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbycomparisonPdgPartitionwhen) -> Self {
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
        val: TopPartitionbycomparisonPdgPartitiontargettype,
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
        val: TopPartitionbycomparisonPdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(
        mut self,
        val: TopPartitionbycomparisonPdgPartitionframe,
    ) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbycomparisonPdgPartitionsort) -> Self {
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
        val: TopPartitionbycomparisonPdgPartitionsortdirection,
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
    pub fn with_comparepairname(mut self, val: &str) -> Self {
        self.params.insert(
            "comparepairname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_comparepairname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comparepairname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultname(mut self, val: &str) -> Self {
        self.params.insert(
            "resultname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resultname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resultname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_partitionsecondary(mut self, val: bool) -> Self {
        self.params.insert(
            "partitionsecondary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_partitionsecondary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionsecondary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for TopPartitionbycomparison {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbycomparison"
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
pub enum TopPartitionbyexpressionPdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyexpressionPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyexpressionPdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyexpressionPdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyexpressionPdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyexpressionPdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyexpressionPdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbyexpression {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbyexpression {
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
    pub fn with_prefilter(mut self, val: i32) -> Self {
        self.params.insert(
            "prefilter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_prefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partition(mut self, val: i32) -> Self {
        self.params.insert(
            "partition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_partition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbyexpressionPdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbyexpressionPdgPartitionwhen) -> Self {
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
        val: TopPartitionbyexpressionPdgPartitiontargettype,
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
        val: TopPartitionbyexpressionPdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(
        mut self,
        val: TopPartitionbyexpressionPdgPartitionframe,
    ) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbyexpressionPdgPartitionsort) -> Self {
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
        val: TopPartitionbyexpressionPdgPartitionsortdirection,
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
    pub fn with_useprefilter(mut self, val: bool) -> Self {
        self.params.insert(
            "useprefilter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useprefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useprefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for TopPartitionbyexpression {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbyexpression"
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
pub enum TopPartitionbyframeCreatepartitionsfor {
    IndividualFrames = 0,
    DistinctFrameRanges = 1,
    FrameSequences = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframeFramerounding {
    RoundDown = 0,
    RoundUp = 1,
    RoundNearest = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframeIgnoremissing {
    ReportError = 0,
    IgnoreWorkItem = 1,
    AddWorkItemToAllPartitions = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframeRangemissing {
    ReportError = 0,
    ReportWarning = 1,
    IgnoreWorkItem = 2,
    AddToAllPartitions = 3,
    AddToLastPartition = 4,
    AddToUniquePartition = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframeFiltertype {
    IncludeAllFrames = 0,
    UsingValueRange = 1,
    UsingPattern = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframePdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    /// Add Work Item to Partitions w/ Matching Frame
    AddWorkItemToPartitionsWMatchingFrame = 1,
    AddWorkItemToAllPartitions = 2,
    UseDefaultValue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframeMissingattribute {
    IgnoreItem = 0,
    AddItemToPartitionsWithMatchingFrame = 1,
    AddItemToAllPartitions = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframePdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframePdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframePdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframePdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyframePdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbyframe {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbyframe {
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

    // --- Float2 parameters ---
    pub fn with_filterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "filterrange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_filterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_createpartitionsfor(mut self, val: TopPartitionbyframeCreatepartitionsfor) -> Self {
        self.params.insert(
            "createpartitionsfor".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createpartitionsfor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createpartitionsfor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framerounding(mut self, val: TopPartitionbyframeFramerounding) -> Self {
        self.params.insert(
            "framerounding".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_framerounding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framerounding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ignoremissing(mut self, val: TopPartitionbyframeIgnoremissing) -> Self {
        self.params.insert(
            "ignoremissing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ignoremissing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ignoremissing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangemissing(mut self, val: TopPartitionbyframeRangemissing) -> Self {
        self.params.insert(
            "rangemissing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rangemissing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangemissing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filtertype(mut self, val: TopPartitionbyframeFiltertype) -> Self {
        self.params.insert(
            "filtertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filtertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filtertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionsplitmissing(
        mut self,
        val: TopPartitionbyframePdgPartitionsplitmissing,
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
    pub fn with_missingattribute(mut self, val: TopPartitionbyframeMissingattribute) -> Self {
        self.params.insert(
            "missingattribute".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbyframePdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbyframePdgPartitionwhen) -> Self {
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
        val: TopPartitionbyframePdgPartitiontargettype,
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbyframePdgPartitionsort) -> Self {
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
        val: TopPartitionbyframePdgPartitionsortdirection,
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
    pub fn with_updaterange(mut self, val: &str) -> Self {
        self.params.insert(
            "updaterange".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_updaterange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updaterange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangeattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "rangeattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rangeattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangeattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "filterpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filterpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterpattern".to_string(),
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
    pub fn with_wedgeattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "wedgeattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_wedgeattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wedgeattribute".to_string(),
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
    pub fn with_useupdaterange(mut self, val: bool) -> Self {
        self.params.insert(
            "useupdaterange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useupdaterange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useupdaterange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexfromframe(mut self, val: bool) -> Self {
        self.params.insert(
            "indexfromframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indexfromframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexfromframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_isolate(mut self, val: bool) -> Self {
        self.params.insert(
            "isolate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_isolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "isolate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterinclusive(mut self, val: bool) -> Self {
        self.params.insert(
            "filterinclusive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterinclusive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterinclusive".to_string(),
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
    pub fn with_usewedgeattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "usewedgeattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usewedgeattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usewedgeattribute".to_string(),
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

impl crate::core::types::HoudiniNode for TopPartitionbyframe {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbyframe"
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
pub enum TopPartitionbyindexPrimaryrule {
    All = 0,
    ByIndex = 1,
    None = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexSecondaryrule {
    All = 0,
    ByIndex = 1,
    PrimaryIntersection = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexPdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexPdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexPdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexPdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexPdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyindexPdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbyindex {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbyindex {
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
    pub fn with_primaryrule(mut self, val: TopPartitionbyindexPrimaryrule) -> Self {
        self.params.insert(
            "primaryrule".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_primaryrule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primaryrule".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_secondaryrule(mut self, val: TopPartitionbyindexSecondaryrule) -> Self {
        self.params.insert(
            "secondaryrule".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_secondaryrule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondaryrule".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbyindexPdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbyindexPdgPartitionwhen) -> Self {
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
        val: TopPartitionbyindexPdgPartitiontargettype,
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
        val: TopPartitionbyindexPdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(mut self, val: TopPartitionbyindexPdgPartitionframe) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbyindexPdgPartitionsort) -> Self {
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
        val: TopPartitionbyindexPdgPartitionsortdirection,
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

impl crate::core::types::HoudiniNode for TopPartitionbyindex {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbyindex"
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
pub enum TopPartitionbyiterationExternalitems {
    Ignore = 0,
    AddToAllPartitions = 1,
    ReportWarning = 2,
    ReportError = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPreviousiterations {
    None = 0,
    All = 1,
    FixedCount = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyiterationPdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbyiteration {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbyiteration {
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
    pub fn with_includenumber(mut self, val: i32) -> Self {
        self.params.insert(
            "includenumber".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_includenumber_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includenumber".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_externalitems(mut self, val: TopPartitionbyiterationExternalitems) -> Self {
        self.params.insert(
            "externalitems".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_externalitems_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalitems".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previousiterations(
        mut self,
        val: TopPartitionbyiterationPreviousiterations,
    ) -> Self {
        self.params.insert(
            "previousiterations".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_previousiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previousiterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbyiterationPdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbyiterationPdgPartitionwhen) -> Self {
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
        val: TopPartitionbyiterationPdgPartitiontargettype,
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
        val: TopPartitionbyiterationPdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(
        mut self,
        val: TopPartitionbyiterationPdgPartitionframe,
    ) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbyiterationPdgPartitionsort) -> Self {
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
        val: TopPartitionbyiterationPdgPartitionsortdirection,
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

impl crate::core::types::HoudiniNode for TopPartitionbyiteration {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbyiteration"
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
pub enum TopPartitionbynodePartitiontype {
    InputNode = 0,
    InputWorkItemTuple = 1,
    InputWorkItemCombination = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodeSortnodes {
    Name = 0,
    InputIndex = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodePdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodePdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodePdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodePdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodePdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodePdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbynodePdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbynode {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbynode {
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
    pub fn with_sortkey(mut self, val: i32) -> Self {
        self.params.insert(
            "sortkey".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sortkey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sortkey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_partitiontype(mut self, val: TopPartitionbynodePartitiontype) -> Self {
        self.params.insert(
            "partitiontype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_partitiontype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitiontype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sortnodes(mut self, val: TopPartitionbynodeSortnodes) -> Self {
        self.params.insert(
            "sortnodes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sortnodes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sortnodes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbynodePdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbynodePdgPartitionwhen) -> Self {
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
        val: TopPartitionbynodePdgPartitiontargettype,
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
        val: TopPartitionbynodePdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(mut self, val: TopPartitionbynodePdgPartitionframe) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbynodePdgPartitionsort) -> Self {
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
        val: TopPartitionbynodePdgPartitionsortdirection,
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
    pub fn with_usesortkey(mut self, val: bool) -> Self {
        self.params.insert(
            "usesortkey".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesortkey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesortkey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partialpartitions(mut self, val: bool) -> Self {
        self.params.insert(
            "partialpartitions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_partialpartitions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partialpartitions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for TopPartitionbynode {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbynode"
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
pub enum TopPartitionbyrangeRangetype {
    CustomRange = 0,
    Separator = 1,
    FixedPartitionCount = 2,
    FixedPartitionSize = 3,
    /// _separator_
    Separator1 = 4,
    /// First/Last Value
    FirstLastValue = 5,
    /// First/Middle/Last Value
    FirstMiddleLastValue = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangeOrderby {
    WorkItemIndex = 0,
    WorkItemFrame = 1,
    CustomAttribute = 2,
    InputOrder = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangePdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangePdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangePdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangePdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangePdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangePdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbyrangePdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbyrange {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbyrange {
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
    pub fn with_rangecount(mut self, val: i32) -> Self {
        self.params.insert(
            "rangecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rangecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangesize(mut self, val: i32) -> Self {
        self.params.insert(
            "rangesize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upstreamrange(mut self, val: i32) -> Self {
        self.params.insert(
            "upstreamrange".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_upstreamrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upstreamrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partitionrange(mut self, val: i32) -> Self {
        self.params.insert(
            "partitionrange".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_partitionrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partitionrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_leftoffset(mut self, val: i32) -> Self {
        self.params.insert(
            "leftoffset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_leftoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "leftoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rightoffset(mut self, val: i32) -> Self {
        self.params.insert(
            "rightoffset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rightoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rightoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_rangetype(mut self, val: TopPartitionbyrangeRangetype) -> Self {
        self.params.insert(
            "rangetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rangetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_orderby(mut self, val: TopPartitionbyrangeOrderby) -> Self {
        self.params.insert(
            "orderby".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_orderby_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "orderby".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbyrangePdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbyrangePdgPartitionwhen) -> Self {
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
        val: TopPartitionbyrangePdgPartitiontargettype,
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
        val: TopPartitionbyrangePdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(mut self, val: TopPartitionbyrangePdgPartitionframe) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbyrangePdgPartitionsort) -> Self {
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
        val: TopPartitionbyrangePdgPartitionsortdirection,
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
    pub fn with_customattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "customattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_resetindex(mut self, val: bool) -> Self {
        self.params.insert(
            "resetindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resetindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resetindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_consolidate(mut self, val: bool) -> Self {
        self.params.insert(
            "consolidate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_consolidate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "consolidate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

impl crate::core::types::HoudiniNode for TopPartitionbyrange {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbyrange"
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
pub enum TopPartitionbytileTileindexorder {
    RowMajor = 0,
    ColumnMajor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytileSrcattrtype {
    PointsString = 0,
    PointsVector = 1,
    BoundingBox = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytilePdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytilePdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytilePdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytilePdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    AddWorkItemToAllPartitions = 1,
    UseDefaultValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytilePdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytilePdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPartitionbytilePdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPartitionbytile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPartitionbytile {
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

    // --- Float parameters ---
    pub fn with_ptradius(mut self, val: f32) -> Self {
        self.params.insert(
            "ptradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ptradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_tileorigin(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "tileorigin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_tileorigin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tileorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_totalsize(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "totalsize".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_totalsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "totalsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_tileindexorder(mut self, val: TopPartitionbytileTileindexorder) -> Self {
        self.params.insert(
            "tileindexorder".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_tileindexorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tileindexorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srcattrtype(mut self, val: TopPartitionbytileSrcattrtype) -> Self {
        self.params.insert(
            "srcattrtype".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_srcattrtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcattrtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_tilecount(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "tilecount".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_tilecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tilecount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPartitionbytilePdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPartitionbytilePdgPartitionwhen) -> Self {
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
        val: TopPartitionbytilePdgPartitiontargettype,
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
        val: TopPartitionbytilePdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(mut self, val: TopPartitionbytilePdgPartitionframe) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPartitionbytilePdgPartitionsort) -> Self {
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
        val: TopPartitionbytilePdgPartitionsortdirection,
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
    pub fn with_ptsattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "ptsattrname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ptsattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ptsattrname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "minattrname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_minattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minattrname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxattrname(mut self, val: &str) -> Self {
        self.params.insert(
            "maxattrname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_maxattrname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxattrname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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

    // --- Toggle parameters ---
    pub fn with_connectedpts(mut self, val: bool) -> Self {
        self.params.insert(
            "connectedpts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_connectedpts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectedpts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
}

impl crate::core::types::HoudiniNode for TopPartitionbytile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "partitionbytile"
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
pub enum TopPerforcePdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
    /// Cook (Out-of-Process)
    CookOutMinusOfMinusProcess = 2,
    /// Cook (Service)
    CookService = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPerforceOpduring {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
    /// Cook (Out-of-Process)
    CookOutMinusOfMinusProcess = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPerforcePdgUseserviceblock {
    Never = 0,
    IfServiceNameMatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPerforcePerforceop {
    Status = 0,
    Update = 1,
    Sync = 2,
    Edit = 3,
    Submit = 4,
    Add = 5,
    Change = 6,
    PrintVersion = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPerforceSourcefiles {
    FilePattern = 0,
    UpstreamOutputFiles = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPerforcePdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPerforcePdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPerforcePdgCommandtype {
    UseDefault = 0,
    CustomScript = 1,
    CustomCommand = 2,
}

#[derive(Debug, Clone)]
pub struct TopPerforce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPerforce {
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
    pub fn trigger_manageservices(mut self) -> Self {
        self.params.insert(
            "manageservices".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_perforceop(mut self, val: TopPerforcePerforceop) -> Self {
        self.params.insert(
            "perforceop".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_perforceop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perforceop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_pdg_cooktype(mut self, val: TopPerforcePdgCooktype) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opduring(mut self, val: TopPerforceOpduring) -> Self {
        self.params.insert(
            "opduring".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_opduring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opduring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useserviceblock(mut self, val: TopPerforcePdgUseserviceblock) -> Self {
        self.params.insert(
            "pdg_useserviceblock".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_useserviceblock_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useserviceblock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcefiles(mut self, val: TopPerforceSourcefiles) -> Self {
        self.params.insert(
            "sourcefiles".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcefiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefiles".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopPerforcePdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopPerforcePdgWorkitempriority) -> Self {
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
    pub fn with_pdg_commandtype(mut self, val: TopPerforcePdgCommandtype) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_commandtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "filepattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filepattern".to_string(),
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
    pub fn with_changelistchoice(mut self, val: &str) -> Self {
        self.params.insert(
            "changelistchoice".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_changelistchoice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "changelistchoice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_changelistnum(mut self, val: &str) -> Self {
        self.params.insert(
            "changelistnum".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_changelistnum_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "changelistnum".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_changelistdescription(mut self, val: &str) -> Self {
        self.params.insert(
            "changelistdescription".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_changelistdescription_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "changelistdescription".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p4configfile(mut self, val: &str) -> Self {
        self.params.insert(
            "p4configfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_p4configfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p4configfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_perforcebin(mut self, val: &str) -> Self {
        self.params.insert(
            "perforcebin".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_perforcebin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perforcebin".to_string(),
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
    pub fn with_pdg_command(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_command_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_emptychangelist(mut self, val: bool) -> Self {
        self.params.insert(
            "emptychangelist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_emptychangelist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emptychangelist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updateall(mut self, val: bool) -> Self {
        self.params.insert(
            "updateall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updateall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updateall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_forcesync(mut self, val: bool) -> Self {
        self.params.insert(
            "forcesync".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forcesync_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcesync".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridebin(mut self, val: bool) -> Self {
        self.params.insert(
            "overridebin".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridebin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridebin".to_string(),
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

impl crate::core::types::HoudiniNode for TopPerforce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "perforce"
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
pub enum TopPreflightTransferduring {
    Generate = 0,
    Cook = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPreflightFilesource {
    /// Upstream Output File(s)
    UpstreamOutputFileS = 0,
    CustomFilePath = 1,
    FileAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPreflightDestination {
    TempFileDirectory = 0,
    WorkingDirectory = 1,
    ScriptDirectory = 2,
    AssetDirectory = 3,
}

#[derive(Debug, Clone)]
pub struct TopPreflight {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPreflight {
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
    pub fn trigger_preflightscene(mut self) -> Self {
        self.params.insert(
            "preflightscene".to_string(),
            crate::core::types::ParamValue::Button,
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
    pub fn with_transferduring(mut self, val: TopPreflightTransferduring) -> Self {
        self.params.insert(
            "transferduring".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_transferduring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transferduring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filesource_inst(mut self, index1: usize, val: TopPreflightFilesource) -> Self {
        self.params.insert(
            format!("filesource{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filesource_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filesource{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_destination_inst(mut self, index1: usize, val: TopPreflightDestination) -> Self {
        self.params.insert(
            format!("destination{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_destination_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("destination{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_rootdirectory(mut self, val: &str) -> Self {
        self.params.insert(
            "rootdirectory".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rootdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rootdirectory".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filetag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filetag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filetag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filetag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_path_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("path{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_path_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("path{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fileattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("fileattrib{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fileattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("fileattrib{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "outputattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enabled{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enabled{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveroot_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("preserveroot{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveroot_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("preserveroot{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useoutputattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "useoutputattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useoutputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useoutputattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfiles(mut self, val: bool) -> Self {
        self.params.insert(
            "outputfiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputfiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopPreflight {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "preflight"
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
pub struct TopPythonmapper {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPythonmapper {
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

    // --- Button parameters ---
    pub fn trigger_savenodescript(mut self) -> Self {
        self.params.insert(
            "savenodescript".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savenodehda(mut self) -> Self {
        self.params.insert(
            "savenodehda".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- String parameters ---
    pub fn with_mapstatic(mut self, val: &str) -> Self {
        self.params.insert(
            "mapstatic".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapstatic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapstatic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mapdynamic(mut self, val: &str) -> Self {
        self.params.insert(
            "mapdynamic".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mapdynamic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mapdynamic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopPythonmapper {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pythonmapper"
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
pub enum TopPythonpartitionerPdgPartitionmergeoutputs {
    None = 0,
    FirstFile = 1,
    LastFile = 2,
    UniqueFiles = 3,
    AllFiles = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonpartitionerPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonpartitionerPdgPartitiontargettype {
    DirectInputNodes = 0,
    UpstreamStaticNodes = 1,
    CustomTargetNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonpartitionerPdgPartitionsplitmissing {
    IgnoreWorkItem = 0,
    HandleWorkItemInPythonCode = 1,
    AddWorkItemToAllPartitions = 2,
    UseDefaultValue = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonpartitionerPdgPartitionframe {
    None = 0,
    /// First Work Item's Frame
    FirstWorkItemSFrame = 1,
    /// Last Work Item's Frame
    LastWorkItemSFrame = 2,
    LargestFrame = 3,
    SmallestFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonpartitionerPdgPartitionsort {
    None = 0,
    WorkItemIndex = 1,
    InputNodeOrder = 2,
    Attribute = 3,
    WorkItemFrame = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonpartitionerPdgPartitionsortdirection {
    InAscendingOrder = 0,
    InDescendingOrder = 1,
}

#[derive(Debug, Clone)]
pub struct TopPythonpartitioner {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopPythonpartitioner {
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

    // --- Button parameters ---
    pub fn trigger_savenodescript(mut self) -> Self {
        self.params.insert(
            "savenodescript".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savenodehda(mut self) -> Self {
        self.params.insert(
            "savenodehda".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_partitionmergeoutputs(
        mut self,
        val: TopPythonpartitionerPdgPartitionmergeoutputs,
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
    pub fn with_pdg_partitionwhen(mut self, val: TopPythonpartitionerPdgPartitionwhen) -> Self {
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
        val: TopPythonpartitionerPdgPartitiontargettype,
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
        val: TopPythonpartitionerPdgPartitionsplitmissing,
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
    pub fn with_pdg_partitionframe(mut self, val: TopPythonpartitionerPdgPartitionframe) -> Self {
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
    pub fn with_pdg_partitionsort(mut self, val: TopPythonpartitionerPdgPartitionsort) -> Self {
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
        val: TopPythonpartitionerPdgPartitionsortdirection,
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
    pub fn with_partition(mut self, val: &str) -> Self {
        self.params.insert(
            "partition".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_partition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_pdg_partitionsortrequired(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_partitionsortrequired".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_partitionsortrequired_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_partitionsortrequired".to_string(),
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

impl crate::core::types::HoudiniNode for TopPythonpartitioner {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pythonpartitioner"
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
pub enum TopPythonprocessorStatefilter {
    Succeeded = 0,
    Cached = 1,
    Failed = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonprocessorPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonprocessorPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopPythonprocessor {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPythonprocessor {
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
    pub fn trigger_savenodescript(mut self) -> Self {
        self.params.insert(
            "savenodescript".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savenodehda(mut self) -> Self {
        self.params.insert(
            "savenodehda".to_string(),
            crate::core::types::ParamValue::Button,
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
    pub fn with_statefilter(mut self, val: TopPythonprocessorStatefilter) -> Self {
        self.params.insert(
            "statefilter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_statefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "statefilter".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopPythonprocessorPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopPythonprocessorPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_command(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_command_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generate(mut self, val: &str) -> Self {
        self.params.insert(
            "generate".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_generate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regeneratestatic(mut self, val: &str) -> Self {
        self.params.insert(
            "regeneratestatic".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_regeneratestatic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "regeneratestatic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addinternaldependencies(mut self, val: &str) -> Self {
        self.params.insert(
            "addinternaldependencies".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_addinternaldependencies_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addinternaldependencies".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cooktask(mut self, val: &str) -> Self {
        self.params.insert(
            "cooktask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_cooktask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cooktask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_servicename(mut self, val: &str) -> Self {
        self.params.insert(
            "servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
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
    pub fn with_useregeneratestatic(mut self, val: bool) -> Self {
        self.params.insert(
            "useregeneratestatic".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useregeneratestatic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useregeneratestatic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alwaysregenerate(mut self, val: bool) -> Self {
        self.params.insert(
            "alwaysregenerate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alwaysregenerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alwaysregenerate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirtyonregenerate(mut self, val: bool) -> Self {
        self.params.insert(
            "dirtyonregenerate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dirtyonregenerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirtyonregenerate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirtyonincomplete(mut self, val: bool) -> Self {
        self.params.insert(
            "dirtyonincomplete".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dirtyonincomplete_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dirtyonincomplete".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requiresgeneratedinputs(mut self, val: bool) -> Self {
        self.params.insert(
            "requiresgeneratedinputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requiresgeneratedinputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requiresgeneratedinputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requirescookedinputs(mut self, val: bool) -> Self {
        self.params.insert(
            "requirescookedinputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requirescookedinputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirescookedinputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requiresscenefile(mut self, val: bool) -> Self {
        self.params.insert(
            "requiresscenefile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requiresscenefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requiresscenefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requiresinputdata(mut self, val: bool) -> Self {
        self.params.insert(
            "requiresinputdata".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_requiresinputdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requiresinputdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesdynamicbatching(mut self, val: bool) -> Self {
        self.params.insert(
            "usesdynamicbatching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesdynamicbatching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesdynamicbatching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usestatefilter(mut self, val: bool) -> Self {
        self.params.insert(
            "usestatefilter".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usestatefilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usestatefilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clearfailures(mut self, val: bool) -> Self {
        self.params.insert(
            "clearfailures".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clearfailures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clearfailures".to_string(),
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

impl crate::core::types::HoudiniNode for TopPythonprocessor {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pythonprocessor"
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
pub enum TopPythonschedulerPdgWorkitemdatasource {
    TemporaryJsonFile = 0,
    RpcMessage = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonschedulerPdgDeletetempdir {
    Never = 0,
    WhenSchedulerIsDeleted = 1,
    WhenCookCompletes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonschedulerPdgTransfertype {
    FlatCopy = 0,
    RelativeToRoot = 1,
}

#[derive(Debug, Clone)]
pub struct TopPythonscheduler {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPythonscheduler {
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

    // --- Button parameters ---
    pub fn trigger_savenodescript(mut self) -> Self {
        self.params.insert(
            "savenodescript".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pdg_maxtasks(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_maxtasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_maxtasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_maxtasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemdatasource(
        mut self,
        val: TopPythonschedulerPdgWorkitemdatasource,
    ) -> Self {
        self.params.insert(
            "pdg_workitemdatasource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemdatasource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemdatasource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_deletetempdir(mut self, val: TopPythonschedulerPdgDeletetempdir) -> Self {
        self.params.insert(
            "pdg_deletetempdir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_deletetempdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_deletetempdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_transfertype(mut self, val: TopPythonschedulerPdgTransfertype) -> Self {
        self.params.insert(
            "pdg_transfertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_transfertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_transfertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_workingdir(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workingdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workingdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workingdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_transferroot(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_transferroot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_transferroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_transferroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onschedule(mut self, val: &str) -> Self {
        self.params.insert(
            "onschedule".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_onschedule_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onschedule".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onschedulestatic(mut self, val: &str) -> Self {
        self.params.insert(
            "onschedulestatic".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_onschedulestatic_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onschedulestatic".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitasjob(mut self, val: &str) -> Self {
        self.params.insert(
            "submitasjob".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitasjob_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitasjob".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onstart(mut self, val: &str) -> Self {
        self.params.insert(
            "onstart".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_onstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onstop(mut self, val: &str) -> Self {
        self.params.insert(
            "onstop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_onstop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onstop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onstartcook(mut self, val: &str) -> Self {
        self.params.insert(
            "onstartcook".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_onstartcook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onstartcook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onstopcook(mut self, val: &str) -> Self {
        self.params.insert(
            "onstopcook".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_onstopcook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onstopcook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ontick(mut self, val: &str) -> Self {
        self.params.insert(
            "ontick".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ontick_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ontick".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_endsharedserver(mut self, val: &str) -> Self {
        self.params.insert(
            "endsharedserver".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_endsharedserver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endsharedserver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transferfile(mut self, val: &str) -> Self {
        self.params.insert(
            "transferfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transferfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transferfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_getloguri(mut self, val: &str) -> Self {
        self.params.insert(
            "getloguri".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_getloguri_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "getloguri".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_getstatusuri(mut self, val: &str) -> Self {
        self.params.insert(
            "getstatusuri".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_getstatusuri_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "getstatusuri".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_pdg_usemaxtasks(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_usemaxtasks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemaxtasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_usemaxtasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_compressworkitemdata".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_compressworkitemdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_validateoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_validateoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_validateoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_validateoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_waitforfailures(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_waitforfailures".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_waitforfailures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_waitforfailures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopPythonscheduler {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pythonscheduler"
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
pub enum TopPythonscriptInprocess {
    /// Cook (Out-of-Process)
    CookOutMinusOfMinusProcess = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
    Generate = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
    /// Cook (Out-of-Process)
    CookOutMinusOfMinusProcess = 2,
    /// Cook (Service)
    CookService = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPdgUseserviceblock {
    Never = 0,
    IfServiceNameMatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPdgServicereset {
    None = 0,
    ResetClient = 1,
    RestartClient = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPdgServiceresetwhen {
    BeforeCook = 0,
    AfterCook = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
    VirtualEnvironment = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPathsource {
    Custom = 0,
    VirtualEnvrionment = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptBatchtype {
    None = 0,
    FixedSize = 1,
    DynamicSize = 2,
    UpstreamFrameRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptScripttype {
    OncePerSubItem = 0,
    OncePerBatch = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptBatchactivation {
    AllFramesAreReady = 0,
    FirstFrameIsReady = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptScriptsource {
    ScriptCodeParameter = 0,
    ExternalFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptCopyoutputs {
    Never = 0,
    Always = 1,
    /// If Script Doesn't Add Outputs
    IfScriptDoesnTAddOutputs = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptExpectedoutputsfrom {
    None = 0,
    Attribute = 1,
    FileList = 2,
    CustomScript = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonscriptPdgCommandtype {
    UseDefault = 0,
    CustomScript = 1,
    CustomCommand = 2,
}

#[derive(Debug, Clone)]
pub struct TopPythonscript {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPythonscript {
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
    pub fn trigger_manageservices(mut self) -> Self {
        self.params.insert(
            "manageservices".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_batchframes(mut self, val: i32) -> Self {
        self.params.insert(
            "batchframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_batchframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchsize(mut self, val: i32) -> Self {
        self.params.insert(
            "batchsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_batchsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_inprocess(mut self, val: TopPythonscriptInprocess) -> Self {
        self.params.insert(
            "inprocess".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inprocess_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocess".to_string(),
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
    pub fn with_pdg_cooktype(mut self, val: TopPythonscriptPdgCooktype) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useserviceblock(mut self, val: TopPythonscriptPdgUseserviceblock) -> Self {
        self.params.insert(
            "pdg_useserviceblock".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_useserviceblock_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useserviceblock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicereset(mut self, val: TopPythonscriptPdgServicereset) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_servicereset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen(mut self, val: TopPythonscriptPdgServiceresetwhen) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopPythonscriptPythonbin) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathsource_inst(mut self, index1: usize, val: TopPythonscriptPathsource) -> Self {
        self.params.insert(
            format!("pathsource{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pathsource_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pathsource{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchtype(mut self, val: TopPythonscriptBatchtype) -> Self {
        self.params.insert(
            "batchtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_batchtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scripttype(mut self, val: TopPythonscriptScripttype) -> Self {
        self.params.insert(
            "scripttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scripttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scripttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchactivation(mut self, val: TopPythonscriptBatchactivation) -> Self {
        self.params.insert(
            "batchactivation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_batchactivation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchactivation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scriptsource(mut self, val: TopPythonscriptScriptsource) -> Self {
        self.params.insert(
            "scriptsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scriptsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scriptsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyoutputs(mut self, val: TopPythonscriptCopyoutputs) -> Self {
        self.params.insert(
            "copyoutputs".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedoutputsfrom(mut self, val: TopPythonscriptExpectedoutputsfrom) -> Self {
        self.params.insert(
            "expectedoutputsfrom".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_expectedoutputsfrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputsfrom".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopPythonscriptPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopPythonscriptPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_commandtype(mut self, val: TopPythonscriptPdgCommandtype) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_commandtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdgservicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdgservicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgservicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdgservicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonvenv(mut self, val: &str) -> Self {
        self.params.insert(
            "pythonvenv".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pythonvenv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonvenv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonargs(mut self, val: &str) -> Self {
        self.params.insert(
            "pythonargs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pythonargs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonargs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraargs(mut self, val: &str) -> Self {
        self.params.insert(
            "extraargs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extraargs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraargs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_custompath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("custompath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_custompath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("custompath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("venvpath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_venvpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("venvpath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prescript(mut self, val: &str) -> Self {
        self.params.insert(
            "prescript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_prescript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prescript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_script(mut self, val: &str) -> Self {
        self.params.insert(
            "script".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_script_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "script".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scriptfile(mut self, val: &str) -> Self {
        self.params.insert(
            "scriptfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scriptfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scriptfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedoutputattr(mut self, val: &str) -> Self {
        self.params.insert(
            "expectedoutputattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expectedoutputattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedoutputtag(mut self, val: &str) -> Self {
        self.params.insert(
            "expectedoutputtag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expectedoutputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedoutputfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("expectedoutputfile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expectedoutputfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("expectedoutputfile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedoutputtag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("expectedoutputtag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expectedoutputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("expectedoutputtag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedscript(mut self, val: &str) -> Self {
        self.params.insert(
            "expectedscript".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expectedscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedscript".to_string(),
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
    pub fn with_pdg_command(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_command_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_resetpython(mut self, val: bool) -> Self {
        self.params.insert(
            "resetpython".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resetpython_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resetpython".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nativeworkitem(mut self, val: bool) -> Self {
        self.params.insert(
            "nativeworkitem".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nativeworkitem_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nativeworkitem".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usebatchframes(mut self, val: bool) -> Self {
        self.params.insert(
            "usebatchframes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebatchframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebatchframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expandscript(mut self, val: bool) -> Self {
        self.params.insert(
            "expandscript".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandscript".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expandstrings(mut self, val: bool) -> Self {
        self.params.insert(
            "expandstrings".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandstrings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandstrings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useexpectedoutputtag(mut self, val: bool) -> Self {
        self.params.insert(
            "useexpectedoutputtag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexpectedoutputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useexpectedoutputtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useexpectedoutputtag_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useexpectedoutputtag{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexpectedoutputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useexpectedoutputtag{}", index1),
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

impl crate::core::types::HoudiniNode for TopPythonscript {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pythonscript"
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
pub enum TopPythonserverCopyinputs {
    NoIterations = 0,
    FirstIteration = 1,
    AllIterations = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonserverPdgCooktype {
    /// Shared Server (Deprecated)
    SharedServerDeprecated = 0,
    Service = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonserverPdgServicereset {
    None = 0,
    ResetClient = 1,
    RestartClient = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonserverPdgServiceresetwhen {
    BeforeCook = 0,
    AfterCook = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonserverPrototype {
    XmlRpc = 0,
    RawTcp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonserverPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonserverPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopPythonserver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPythonserver {
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

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_commandport(mut self, val: i32) -> Self {
        self.params.insert(
            "commandport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_commandport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "commandport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeout(mut self, val: i32) -> Self {
        self.params.insert(
            "timeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_copyinputs(mut self, val: TopPythonserverCopyinputs) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyinputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopPythonserverPdgCooktype) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicereset(mut self, val: TopPythonserverPdgServicereset) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_servicereset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen(mut self, val: TopPythonserverPdgServiceresetwhen) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_prototype(mut self, val: TopPythonserverPrototype) -> Self {
        self.params.insert(
            "prototype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_prototype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prototype".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopPythonserverPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopPythonserverPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_feedbackpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_feedbackpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalhost(mut self, val: &str) -> Self {
        self.params.insert(
            "externalhost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalhost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalhost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_startupcmdline(mut self, val: &str) -> Self {
        self.params.insert(
            "startupcmdline".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_startupcmdline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startupcmdline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "iterattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_iterattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sizeattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "sizeattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sizeattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizeattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "numattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_numattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
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
    pub fn with_iterationsfromupstream(mut self, val: bool) -> Self {
        self.params.insert(
            "iterationsfromupstream".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iterationsfromupstream_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterationsfromupstream".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loopsequential(mut self, val: bool) -> Self {
        self.params.insert(
            "loopsequential".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loopsequential_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loopsequential".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_feedbackattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackfiles(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_feedbackfiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackfiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalserver(mut self, val: bool) -> Self {
        self.params.insert(
            "externalserver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_externalserver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalserver".to_string(),
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

impl crate::core::types::HoudiniNode for TopPythonserver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pythonserver"
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
pub enum TopPythonvenvPythonbin {
    Hython = 0,
    PdgPython = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonvenvRequirements {
    None = 0,
    RequirementsFile = 1,
    PackageList = 2,
    PackageDictionary = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonvenvFiletype {
    UpstreamOutputFile = 0,
    CustomFilePath = 1,
    String = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonvenvPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    UseCustomExpression = 2,
    NodeDefinesLabel = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonvenvPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    UseCustomExpression = 1,
    NodeDefinesPriority = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopPythonvenvPdgCommandtype {
    UseDefault = 0,
    CustomScript = 1,
    CustomCommand = 2,
}

#[derive(Debug, Clone)]
pub struct TopPythonvenv {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopPythonvenv {
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

    // --- Data parameters ---
    pub fn with_requirementdict(mut self, val: &str) -> Self {
        self.params.insert(
            "requirementdict".to_string(),
            crate::core::types::ParamValue::Data(val.to_string()),
        );
        self
    }
    pub fn with_requirementdict_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirementdict".to_string(),
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
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopPythonvenvPythonbin) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requirements(mut self, val: TopPythonvenvRequirements) -> Self {
        self.params.insert(
            "requirements".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_requirements_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirements".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filetype(mut self, val: TopPythonvenvFiletype) -> Self {
        self.params.insert(
            "filetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filetype".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopPythonvenvPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopPythonvenvPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_commandtype(mut self, val: TopPythonvenvPdgCommandtype) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_commandtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_venvpath(mut self, val: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_venvpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_python(mut self, val: &str) -> Self {
        self.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_python_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "python".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requirementshfs(mut self, val: &str) -> Self {
        self.params.insert(
            "requirementshfs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_requirementshfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirementshfs".to_string(),
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
    pub fn with_customfile(mut self, val: &str) -> Self {
        self.params.insert(
            "customfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filestring(mut self, val: &str) -> Self {
        self.params.insert(
            "filestring".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filestring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filestring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_requirementlist(mut self, val: &str) -> Self {
        self.params.insert(
            "requirementlist".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_requirementlist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "requirementlist".to_string(),
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
    pub fn with_pdg_command(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_command_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_venvsymlink(mut self, val: bool) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvsymlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvsymlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepipcache(mut self, val: bool) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepipcache_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepipcache".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvlock(mut self, val: bool) -> Self {
        self.params.insert(
            "venvlock".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvlock_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvlock".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userequirementshfs(mut self, val: bool) -> Self {
        self.params.insert(
            "userequirementshfs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userequirementshfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userequirementshfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_venvoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "venvoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_venvoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "venvoutput".to_string(),
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

impl crate::core::types::HoudiniNode for TopPythonvenv {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "pythonvenv"
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
