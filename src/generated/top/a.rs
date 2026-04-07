#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributearrayNameconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    Separator = 2,
    UpdateExistingAttributeValue = 3,
    AppendToExistingAttribute = 4,
    PrependToExistingAttribute = 5,
    /// _separator_
    Separator1 = 6,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 7,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributearrayCreatefrom {
    ValuePattern = 0,
    AttributePattern = 1,
    ExistingArrayAttribute = 2,
    OutputFiles = 3,
    ArrayValues = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributearrayArraytype {
    String = 0,
    Integer = 1,
    Float = 2,
    Dictionary = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributearrayCopyvalues {
    TakeAll = 0,
    TakeSlice = 1,
    TakeFirst = 2,
    TakeLast = 3,
    TakeMiddle = 4,
    TakeAtIndex = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributearrayOnindexerror {
    GenerateError = 0,
    GenerateWarning = 1,
}

#[derive(Debug, Clone)]
pub struct TopAttributearray {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributearray {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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

    // --- Int parameters ---
    pub fn with_condition(mut self, val: i32) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
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
    pub fn with_slicestart(mut self, val: i32) -> Self {
        self.params.insert(
            "slicestart".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slicestart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slicestart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceend(mut self, val: i32) -> Self {
        self.params.insert(
            "sliceend".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sliceend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slicestep(mut self, val: i32) -> Self {
        self.params.insert(
            "slicestep".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slicestep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slicestep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_takecount(mut self, val: i32) -> Self {
        self.params.insert(
            "takecount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_takecount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "takecount".to_string(),
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
    pub fn with_nameconflict(mut self, val: TopAttributearrayNameconflict) -> Self {
        self.params.insert(
            "nameconflict".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_nameconflict_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameconflict".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfrom(mut self, val: TopAttributearrayCreatefrom) -> Self {
        self.params.insert(
            "createfrom".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_createfrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfrom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arraytype(mut self, val: TopAttributearrayArraytype) -> Self {
        self.params.insert(
            "arraytype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_arraytype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arraytype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyvalues(mut self, val: TopAttributearrayCopyvalues) -> Self {
        self.params.insert(
            "copyvalues".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyvalues_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyvalues".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onindexerror(mut self, val: TopAttributearrayOnindexerror) -> Self {
        self.params.insert(
            "onindexerror".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_onindexerror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onindexerror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_attribname(mut self, val: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
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
    pub fn with_dictvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dictvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dictvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "attribpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribvaluepattern(mut self, val: &str) -> Self {
        self.params.insert(
            "attribvaluepattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribvaluepattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribvaluepattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existingattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "existingattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_existingattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existingattribname".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_usecondition(mut self, val: bool) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceuseend(mut self, val: bool) -> Self {
        self.params.insert(
            "sliceuseend".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sliceuseend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceuseend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sorted(mut self, val: bool) -> Self {
        self.params.insert(
            "sorted".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sorted_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sorted".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversed(mut self, val: bool) -> Self {
        self.params.insert(
            "reversed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributearray {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributearray"
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
pub enum TopAttributeclassifyPdgWorkitemgeneration {
    EachUpstreamItemIsGenerated = 0,
    AllUpstreamItemsAreGenerated = 1,
    Automatic = 2,
    AllUpstreamItemsAreCooked = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributeclassifyClasstype {
    EachMatchingAttribute = 0,
    AllMatchingAttributes = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributeclassifyUnmatched {
    ExcludeWorkItem = 0,
    ExcludeClassAttribute = 1,
    UseDefaultClass = 2,
    UseUnmatchedClass = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributeclassifyAttribsource {
    AttributePattern = 0,
    StringAttribute = 1,
    AttributeList = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributeclassifyMissing {
    Ignore = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone)]
pub struct TopAttributeclassify {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributeclassify {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_classifywhen(mut self, val: i32) -> Self {
        self.params.insert(
            "classifywhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_classifywhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classifywhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultclass(mut self, val: i32) -> Self {
        self.params.insert(
            "defaultclass".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_defaultclass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultclass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(
        mut self,
        val: TopAttributeclassifyPdgWorkitemgeneration,
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
    pub fn with_classtype(mut self, val: TopAttributeclassifyClasstype) -> Self {
        self.params.insert(
            "classtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_classtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unmatched(mut self, val: TopAttributeclassifyUnmatched) -> Self {
        self.params.insert(
            "unmatched".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_unmatched_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unmatched".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribsource(mut self, val: TopAttributeclassifyAttribsource) -> Self {
        self.params.insert(
            "attribsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missing(mut self, val: TopAttributeclassifyMissing) -> Self {
        self.params.insert(
            "missing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_classattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "classattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_classattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "classattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "attribpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribname(mut self, val: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attriblistname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("attriblistname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attriblistname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("attriblistname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useclassifywhen(mut self, val: bool) -> Self {
        self.params.insert(
            "useclassifywhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useclassifywhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useclassifywhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributeclassify {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributeclassify"
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
pub enum TopAttributecopyPdgDynamicpartition {
    Never = 0,
    BeforeCopying = 1,
    BeforeMatching = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecopyMatchby {
    WorkItemIndex = 0,
    Attribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecopyMatchtype {
    Integer = 0,
    Float = 1,
    String = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecopyCopyop {
    KeepExisting = 0,
    OverwriteValueAndType = 1,
    OverwriteValue = 2,
    AppendValue = 3,
    PrependValue = 4,
    UpdateValue = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecopyCopyresults {
    None = 0,
    RealOutputsOnly = 1,
    RealAndExpectedOutputs = 2,
}

#[derive(Debug, Clone)]
pub struct TopAttributecopy {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributecopy {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Workitems to copy attributes to"
    pub fn set_input_workitems_to_copy_attributes_to<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Workitems to copy attributes to" and specifies the output index of the target node.
    pub fn set_input_workitems_to_copy_attributes_to_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Workitems to copy attributes from"
    pub fn set_input_workitems_to_copy_attributes_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Workitems to copy attributes from" and specifies the output index of the target node.
    pub fn set_input_workitems_to_copy_attributes_from_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_dynamicpartition(mut self, val: TopAttributecopyPdgDynamicpartition) -> Self {
        self.params.insert(
            "pdg_dynamicpartition".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_dynamicpartition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_dynamicpartition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchby(mut self, val: TopAttributecopyMatchby) -> Self {
        self.params.insert(
            "matchby".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchby_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchby".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchtype(mut self, val: TopAttributecopyMatchtype) -> Self {
        self.params.insert(
            "matchtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyop(mut self, val: TopAttributecopyCopyop) -> Self {
        self.params.insert(
            "copyop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyresults(mut self, val: TopAttributecopyCopyresults) -> Self {
        self.params.insert(
            "copyresults".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyresults_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyresults".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_matchname(mut self, val: &str) -> Self {
        self.params.insert(
            "matchname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyname(mut self, val: &str) -> Self {
        self.params.insert(
            "copyname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_copyname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyall(mut self, val: bool) -> Self {
        self.params.insert(
            "copyall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_copyall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_excludeunmatched(mut self, val: bool) -> Self {
        self.params.insert(
            "excludeunmatched".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_excludeunmatched_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "excludeunmatched".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributecopy {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributecopy"
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
pub enum TopAttributecreateEmptyinput {
    CreateWorkItem = 0,
    Ignore = 1,
    ReportWarning = 2,
    ReportError = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateCopyoutputs {
    Never = 0,
    Always = 1,
    /// If Node Doesn't Add Outputs
    IfNodeDoesnTAddOutputs = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreatePdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateStringconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttributeValue = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateStringscope {
    WorkItem = 0,
    /// Graph (Bound)
    GraphBound = 1,
    /// Graph (Global)
    GraphGlobal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateIntconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttributeValue = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateIntscope {
    WorkItem = 0,
    /// Graph (Bound)
    GraphBound = 1,
    /// Graph (Global)
    GraphGlobal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateFloatconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttributeValue = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateFloatscope {
    WorkItem = 0,
    /// Graph (Bound)
    GraphBound = 1,
    /// Graph (Global)
    GraphGlobal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateDictconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttributeValue = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateDictscope {
    WorkItem = 0,
    /// Graph (Bound)
    GraphBound = 1,
    /// Graph (Global)
    GraphGlobal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreatePyobjectconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttributeValue = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreatePyobjectscope {
    WorkItem = 0,
    /// Graph (Bound)
    GraphBound = 1,
    /// Graph (Global)
    GraphGlobal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateResultmenu {
    OutputFile = 0,
    FileAttribute = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateResultconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttributeValue = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateResultscope {
    WorkItem = 0,
    /// Graph (Bound)
    GraphBound = 1,
    /// Graph (Global)
    GraphGlobal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateResultcheckfilepath {
    NoFilePathValidation = 0,
    ErrorOnMissingFile = 1,
    WarningOnMissingFile = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributecreateCheckpathwhen {
    WorkItemIsGenerated = 0,
    WorkItemIsCooked = 1,
}

#[derive(Debug, Clone)]
pub struct TopAttributecreate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributecreate {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_framevalue(mut self, val: f32) -> Self {
        self.params.insert(
            "framevalue".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_framevalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framevalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_floatvalue_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.params.insert(
            format!("floatvalue{}", index1),
            crate::core::types::ParamValue::Float4(val),
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

    // --- Int parameters ---
    pub fn with_condition(mut self, val: i32) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
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
    pub fn with_indexvalue(mut self, val: i32) -> Self {
        self.params.insert(
            "indexvalue".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_indexvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_priorityvalue(mut self, val: i32) -> Self {
        self.params.insert(
            "priorityvalue".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_priorityvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "priorityvalue".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stringindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("stringindex{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stringindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stringindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("intsize{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_intsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intsize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatsize_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("floatsize{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_floatsize_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatsize{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("dictindex{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_dictindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int4 parameters ---
    pub fn with_intvalue_inst(mut self, index1: usize, val: [i32; 4]) -> Self {
        self.params.insert(
            format!("intvalue{}", index1),
            crate::core::types::ParamValue::Int4(val),
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
    pub fn with_emptyinput(mut self, val: TopAttributecreateEmptyinput) -> Self {
        self.params.insert(
            "emptyinput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_emptyinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emptyinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyoutputs(mut self, val: TopAttributecreateCopyoutputs) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopAttributecreatePdgWorkitempriority) -> Self {
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
    pub fn with_stringconflict_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateStringconflict,
    ) -> Self {
        self.params.insert(
            format!("stringconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stringconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stringconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stringscope_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateStringscope,
    ) -> Self {
        self.params.insert(
            format!("stringscope{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stringscope_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stringscope{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intconflict_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateIntconflict,
    ) -> Self {
        self.params.insert(
            format!("intconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_intconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intscope_inst(mut self, index1: usize, val: TopAttributecreateIntscope) -> Self {
        self.params.insert(
            format!("intscope{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_intscope_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intscope{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatconflict_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateFloatconflict,
    ) -> Self {
        self.params.insert(
            format!("floatconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_floatconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatscope_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateFloatscope,
    ) -> Self {
        self.params.insert(
            format!("floatscope{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_floatscope_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatscope{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictconflict_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateDictconflict,
    ) -> Self {
        self.params.insert(
            format!("dictconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dictconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictscope_inst(mut self, index1: usize, val: TopAttributecreateDictscope) -> Self {
        self.params.insert(
            format!("dictscope{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dictscope_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictscope{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pyobjectconflict_inst(
        mut self,
        index1: usize,
        val: TopAttributecreatePyobjectconflict,
    ) -> Self {
        self.params.insert(
            format!("pyobjectconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pyobjectconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pyobjectconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pyobjectscope_inst(
        mut self,
        index1: usize,
        val: TopAttributecreatePyobjectscope,
    ) -> Self {
        self.params.insert(
            format!("pyobjectscope{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pyobjectscope_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pyobjectscope{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultmenu_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateResultmenu,
    ) -> Self {
        self.params.insert(
            format!("resultmenu{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resultmenu_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultmenu{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultconflict_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateResultconflict,
    ) -> Self {
        self.params.insert(
            format!("resultconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resultconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultscope_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateResultscope,
    ) -> Self {
        self.params.insert(
            format!("resultscope{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resultscope_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultscope{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultcheckfilepath_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateResultcheckfilepath,
    ) -> Self {
        self.params.insert(
            format!("resultcheckfilepath{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resultcheckfilepath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultcheckfilepath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpathwhen_inst(
        mut self,
        index1: usize,
        val: TopAttributecreateCheckpathwhen,
    ) -> Self {
        self.params.insert(
            format!("checkpathwhen{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpathwhen_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("checkpathwhen{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_stringname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("stringname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stringname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stringname{}", index1),
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
    pub fn with_intname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("intname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_intname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("floatname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_floatname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dictname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dictname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("dictvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dictvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pyobjectname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pyobjectname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pyobjectname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pyobjectname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pyobjectexpression_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pyobjectexpression{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pyobjectexpression_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pyobjectexpression{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultattrname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resultattrname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resultattrname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultattrname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resultvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resultvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resulttag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resulttag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resulttag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resulttag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existsattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("existsattrib{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_existsattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("existsattrib{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usecondition(mut self, val: bool) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useindex(mut self, val: bool) -> Self {
        self.params.insert(
            "useindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useframe(mut self, val: bool) -> Self {
        self.params.insert(
            "useframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepriority(mut self, val: bool) -> Self {
        self.params.insert(
            "usepriority".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stringenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("stringenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stringenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stringenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usestringindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usestringindex{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usestringindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usestringindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_intenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("intenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_intenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("floatenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_floatenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("dictenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dictenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedictindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usedictindex{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedictindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usedictindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictinvalid_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("dictinvalid{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dictinvalid_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("dictinvalid{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pyobjectenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("pyobjectenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pyobjectenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pyobjectenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resultenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resultenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultown_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resultown{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resultown_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultown{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resultcopy_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resultcopy{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resultcopy_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resultcopy{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributecreate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributecreate"
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
pub enum TopAttributedeleteDeleteusing {
    AttributeList = 0,
    AttributePattern = 1,
    StringAttributeList = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributedeleteMissingattribute {
    None = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributedeletePatternscope {
    WorkItem = 0,
    Graph = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributedeleteScope {
    Automatic = 0,
    WorkItem = 1,
    Graph = 2,
}

#[derive(Debug, Clone)]
pub struct TopAttributedelete {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributedelete {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_condition(mut self, val: i32) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_deleteusing(mut self, val: TopAttributedeleteDeleteusing) -> Self {
        self.params.insert(
            "deleteusing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_deleteusing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deleteusing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missingattribute(mut self, val: TopAttributedeleteMissingattribute) -> Self {
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
    pub fn with_patternscope(mut self, val: TopAttributedeletePatternscope) -> Self {
        self.params.insert(
            "patternscope".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_patternscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "patternscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope_inst(mut self, index1: usize, val: TopAttributedeleteScope) -> Self {
        self.params.insert(
            format!("scope{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_scope_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("scope{}", index1),
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
    pub fn with_stringattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "stringattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stringattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stringattribname".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_usecondition(mut self, val: bool) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deleteall(mut self, val: bool) -> Self {
        self.params.insert(
            "deleteall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_deleteall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deleteall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyinputs(mut self, val: bool) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_useindex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useindex{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useindex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributedelete {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributedelete"
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
pub enum TopAttributedictionaryExistingattrib {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttribute = 2,
    /// Warn on Type Mis-match
    WarnOnTypeMisMinusMatch = 3,
    /// Error on Type Mis-match
    ErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributedictionaryType {
    Integer = 0,
    Float = 1,
    String = 2,
    IntegerArray = 3,
    FloatArray = 4,
    StringArray = 5,
    Attribute = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributedictionaryExportexisting {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttribute = 2,
    /// Warn on Type Mis-match
    WarnOnTypeMisMinusMatch = 3,
    /// Error on Type Mis-match
    ErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone)]
pub struct TopAttributedictionary {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributedictionary {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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

    // --- Int parameters ---
    pub fn with_enablewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "enablewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enablewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_index(mut self, val: i32) -> Self {
        self.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
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
    pub fn with_stride_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("stride{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stride_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stride{}", index1),
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
    pub fn with_existingattrib(mut self, val: TopAttributedictionaryExistingattrib) -> Self {
        self.params.insert(
            "existingattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_existingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: TopAttributedictionaryType) -> Self {
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
    pub fn with_exportexisting(mut self, val: TopAttributedictionaryExportexisting) -> Self {
        self.params.insert(
            "exportexisting".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exportexisting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportexisting".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mergeattribspattern(mut self, val: &str) -> Self {
        self.params.insert(
            "mergeattribspattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mergeattribspattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeattribspattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mergedictspattern(mut self, val: &str) -> Self {
        self.params.insert(
            "mergedictspattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mergedictspattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergedictspattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removekeyspattern(mut self, val: &str) -> Self {
        self.params.insert(
            "removekeyspattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_removekeyspattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removekeyspattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_key_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("key{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_key_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("key{}", index1),
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
    pub fn with_arrayvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("arrayvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_arrayvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("arrayvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("attributename{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("attributename{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keyattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("keyattrib{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_keyattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("keyattrib{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valueattrib_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("valueattrib{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_valueattrib_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("valueattrib{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportkeys(mut self, val: &str) -> Self {
        self.params.insert(
            "exportkeys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportkeys_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportkeys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exportpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "exportpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_exportpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exportpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useenablewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "useenablewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useenablewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useenablewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useindex(mut self, val: bool) -> Self {
        self.params.insert(
            "useindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemergeattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "usemergeattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemergeattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemergeattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemergedicts(mut self, val: bool) -> Self {
        self.params.insert(
            "usemergedicts".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemergedicts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemergedicts".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useremovekeys(mut self, val: bool) -> Self {
        self.params.insert(
            "useremovekeys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useremovekeys_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useremovekeys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablekey_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enablekey{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablekey_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enablekey{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arrayinclusive_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("arrayinclusive{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_arrayinclusive_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("arrayinclusive{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablepair_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enablepair{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepair_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enablepair{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableexportkeys(mut self, val: bool) -> Self {
        self.params.insert(
            "enableexportkeys".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableexportkeys_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableexportkeys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableexport(mut self, val: bool) -> Self {
        self.params.insert(
            "enableexport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableexport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableexport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributedictionary {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributedictionary"
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
pub enum TopAttributefromfileUseregex {
    SimplePattern = 0,
    RegularExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributefromfileParseissues {
    None = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone)]
pub struct TopAttributefromfile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributefromfile {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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
    pub fn with_useregex(mut self, val: TopAttributefromfileUseregex) -> Self {
        self.params.insert(
            "useregex".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_useregex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useregex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parseissues(mut self, val: TopAttributefromfileParseissues) -> Self {
        self.params.insert(
            "parseissues".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parseissues_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parseissues".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_workitemdelim(mut self, val: &str) -> Self {
        self.params.insert(
            "workitemdelim".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_workitemdelim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "workitemdelim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribmatchstring(mut self, val: &str) -> Self {
        self.params.insert(
            "attribmatchstring".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribmatchstring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribmatchstring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_infertype(mut self, val: bool) -> Self {
        self.params.insert(
            "infertype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_infertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "infertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createnomatch(mut self, val: bool) -> Self {
        self.params.insert(
            "createnomatch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createnomatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createnomatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributefromfile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributefromfile"
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
pub enum TopAttributefromparametersEvaluationtime {
    WorkItemFrame = 0,
    NetworkEvaluationTime = 1,
    CustomTime = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributefromparametersExistingattrib {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttribute = 2,
    /// Warn on Type Mis-match
    WarnOnTypeMisMinusMatch = 3,
    /// Error on Type Mis-match
    ErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone)]
pub struct TopAttributefromparameters {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributefromparameters {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_customtime(mut self, val: f32) -> Self {
        self.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_customtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customtime".to_string(),
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
    pub fn with_evaluationtime(mut self, val: TopAttributefromparametersEvaluationtime) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_evaluationtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_existingattrib(mut self, val: TopAttributefromparametersExistingattrib) -> Self {
        self.params.insert(
            "existingattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_existingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_nodepath(mut self, val: &str) -> Self {
        self.params.insert(
            "nodepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nodepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nodepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filter(mut self, val: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribute(mut self, val: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_evaluate(mut self, val: bool) -> Self {
        self.params.insert(
            "evaluate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_evaluate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributefromparameters {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributefromparameters"
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
pub enum TopAttributefromstringOperation {
    MatchByPattern = 0,
    SplitByDelimiter = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributefromstringUseregex {
    SimplePattern = 0,
    RegularExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributefromstringDelimitertype {
    String = 0,
    CharacterSet = 1,
    RegularExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributefromstringStorage {
    StringArray = 0,
    SeparateWorkItems = 1,
}

#[derive(Debug, Clone)]
pub struct TopAttributefromstring {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributefromstring {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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
    pub fn with_operation(mut self, val: TopAttributefromstringOperation) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useregex(mut self, val: TopAttributefromstringUseregex) -> Self {
        self.params.insert(
            "useregex".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_useregex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useregex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delimitertype(mut self, val: TopAttributefromstringDelimitertype) -> Self {
        self.params.insert(
            "delimitertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_delimitertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delimitertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_storage(mut self, val: TopAttributefromstringStorage) -> Self {
        self.params.insert(
            "storage".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_storage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "storage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcestring(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcestring".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcestring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcestring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchstring(mut self, val: &str) -> Self {
        self.params.insert(
            "matchstring".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchstring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchstring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delimiter(mut self, val: &str) -> Self {
        self.params.insert(
            "delimiter".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_delimiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delimiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "splitattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_splitattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_countattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "countattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_countattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "countattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "indexattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_indexattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_infertype(mut self, val: bool) -> Self {
        self.params.insert(
            "infertype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_infertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "infertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createnomatch(mut self, val: bool) -> Self {
        self.params.insert(
            "createnomatch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createnomatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createnomatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createsplit(mut self, val: bool) -> Self {
        self.params.insert(
            "createsplit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createsplit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createsplit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcount(mut self, val: bool) -> Self {
        self.params.insert(
            "createcount".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createindex(mut self, val: bool) -> Self {
        self.params.insert(
            "createindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trimspace(mut self, val: bool) -> Self {
        self.params.insert(
            "trimspace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trimspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trimspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributefromstring {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributefromstring"
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
pub enum TopAttributepromotePdgWorkitemgeneration {
    EachUpstreamItemIsGenerated = 0,
    AllUpstreamItemsAreGenerated = 1,
    Automatic = 2,
    AllUpstreamItemsAreCooked = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributepromoteFrom {
    WorkItemAttribute = 0,
    /// Input File(s)
    InputFileS = 1,
    IntrinsicField = 2,
    GlobalAttribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributepromoteFromintrinsic {
    Id = 0,
    Index = 1,
    Priority = 2,
    Name = 3,
    Label = 4,
    Separator = 5,
    Frame = 6,
    FrameStep = 7,
    /// _separator_
    Separator1 = 8,
    CookTime = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributepromoteTo {
    WorkItemAttribute = 0,
    /// Output File(s)
    OutputFileS = 1,
    GlobalAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributepromoteToconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    AppendToExistingAttribute = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone)]
pub struct TopAttributepromote {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributepromote {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(
        mut self,
        val: TopAttributepromotePdgWorkitemgeneration,
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
    pub fn with_from_inst(mut self, index1: usize, val: TopAttributepromoteFrom) -> Self {
        self.params.insert(
            format!("from{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_from_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("from{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fromintrinsic_inst(
        mut self,
        index1: usize,
        val: TopAttributepromoteFromintrinsic,
    ) -> Self {
        self.params.insert(
            format!("fromintrinsic{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fromintrinsic_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("fromintrinsic{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_to_inst(mut self, index1: usize, val: TopAttributepromoteTo) -> Self {
        self.params.insert(
            format!("to{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_to_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("to{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toconflict_inst(
        mut self,
        index1: usize,
        val: TopAttributepromoteToconflict,
    ) -> Self {
        self.params.insert(
            format!("toconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_toconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("toconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_fromname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("fromname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fromname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("fromname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fromtag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("fromtag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fromtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("fromtag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("toname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_toname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("toname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_totag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("totag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_totag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("totag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_delete_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("delete{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delete_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("delete{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletoname_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enabletoname{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletoname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enabletoname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tobind_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("tobind{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tobind_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("tobind{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletotag_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enabletotag{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletotag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enabletotag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toownership_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("toownership{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_toownership_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("toownership{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributepromote {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributepromote"
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
pub enum TopAttributerandomizeNameconflict {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    Separator = 2,
    UpdateExistingAttributeValue = 3,
    AppendToExistingAttribute = 4,
    PrependToExistingAttribute = 5,
    /// _separator_
    Separator1 = 6,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 7,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributerandomizeArraytype {
    Integer = 0,
    Float = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributerandomizeRoundtype {
    RoundNearest = 0,
    RoundUp = 1,
    RoundDown = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributerandomizeRanddist {
    /// Uniform (Continuous)
    UniformContinuous = 0,
    /// Uniform (Discrete)
    UniformDiscrete = 1,
    TwoValues = 2,
    Normal = 3,
    Exponential = 4,
    /// Log-Normal
    LogMinusNormal = 5,
    CustomDiscrete = 6,
}

#[derive(Debug, Clone)]
pub struct TopAttributerandomize {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributerandomize {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_uniformcontmin(mut self, val: f32) -> Self {
        self.params.insert(
            "uniformcontmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uniformcontmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformcontmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformcontmax(mut self, val: f32) -> Self {
        self.params.insert(
            "uniformcontmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uniformcontmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformcontmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuea(mut self, val: f32) -> Self {
        self.params.insert(
            "valuea".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_valuea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "valuea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valueb(mut self, val: f32) -> Self {
        self.params.insert(
            "valueb".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_valueb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "valueb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_probvalueb(mut self, val: f32) -> Self {
        self.params.insert(
            "probvalueb".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_probvalueb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "probvalueb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normmedian(mut self, val: f32) -> Self {
        self.params.insert(
            "normmedian".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normmedian_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normmedian".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normstdev(mut self, val: f32) -> Self {
        self.params.insert(
            "normstdev".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normstdev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normstdev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expmedian(mut self, val: f32) -> Self {
        self.params.insert(
            "expmedian".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expmedian_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expmedian".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lognormmedian(mut self, val: f32) -> Self {
        self.params.insert(
            "lognormmedian".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lognormmedian_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lognormmedian".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lognormstdev(mut self, val: f32) -> Self {
        self.params.insert(
            "lognormstdev".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_lognormstdev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lognormstdev".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("value{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("value{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_weight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("weight{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("weight{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "minlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "maxlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_condition(mut self, val: i32) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randseed(mut self, val: i32) -> Self {
        self.params.insert(
            "randseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_randseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arraysize(mut self, val: i32) -> Self {
        self.params.insert(
            "arraysize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_arraysize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arraysize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformdiscmin(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiscmin".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiscmin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiscmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformdiscmax(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiscmax".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiscmax_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiscmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformdiscstep(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiscstep".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiscstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiscstep".to_string(),
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
    pub fn with_nameconflict(mut self, val: TopAttributerandomizeNameconflict) -> Self {
        self.params.insert(
            "nameconflict".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_nameconflict_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameconflict".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_arraytype(mut self, val: TopAttributerandomizeArraytype) -> Self {
        self.params.insert(
            "arraytype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_arraytype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "arraytype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roundtype(mut self, val: TopAttributerandomizeRoundtype) -> Self {
        self.params.insert(
            "roundtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_roundtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roundtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_randdist(mut self, val: TopAttributerandomizeRanddist) -> Self {
        self.params.insert(
            "randdist".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_randdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_attribname(mut self, val: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usecondition(mut self, val: bool) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_toggleminlimit(mut self, val: bool) -> Self {
        self.params.insert(
            "toggleminlimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_toggleminlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "toggleminlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_togglemaxlimit(mut self, val: bool) -> Self {
        self.params.insert(
            "togglemaxlimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_togglemaxlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "togglemaxlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sorted(mut self, val: bool) -> Self {
        self.params.insert(
            "sorted".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sorted_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sorted".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reversed(mut self, val: bool) -> Self {
        self.params.insert(
            "reversed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reversed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reversed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablestats(mut self, val: bool) -> Self {
        self.params.insert(
            "enablestats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablestats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributerandomize {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributerandomize"
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
pub enum TopAttributereducePdgWorkitemgeneration {
    EachUpstreamItemIsGenerated = 0,
    AllUpstreamItemsAreGenerated = 1,
    Automatic = 2,
    AllUpstreamItemsAreCooked = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopAttributereduceMissing {
    Ignore = 0,
    GenerateWarning = 1,
    GenerateError = 2,
}

#[derive(Debug, Clone)]
pub struct TopAttributereduce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributereduce {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_condition(mut self, val: i32) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stride_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("stride{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_stride_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("stride{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(
        mut self,
        val: TopAttributereducePdgWorkitemgeneration,
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
    pub fn with_missing(mut self, val: TopAttributereduceMissing) -> Self {
        self.params.insert(
            "missing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_operation_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("operation{}", index1),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_operation_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("operation{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_attribname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("attribname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("attribname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usecondition(mut self, val: bool) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecondition".to_string(),
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
    pub fn with_usestride_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usestride{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usestride_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usestride{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributereduce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributereduce"
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
pub enum TopAttributerenameOnattributenotfound {
    ReportError = 0,
    ReportWarning = 1,
    Ignore = 2,
}

#[derive(Debug, Clone)]
pub struct TopAttributerename {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributerename {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_condition(mut self, val: i32) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_condition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_priorityvalue(mut self, val: i32) -> Self {
        self.params.insert(
            "priorityvalue".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_priorityvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "priorityvalue".to_string(),
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
    pub fn with_onattributenotfound(mut self, val: TopAttributerenameOnattributenotfound) -> Self {
        self.params.insert(
            "onattributenotfound".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_onattributenotfound_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onattributenotfound".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_renamefrom_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("renamefrom{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renamefrom_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("renamefrom{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renameto_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("renameto{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renameto_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("renameto{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usecondition(mut self, val: bool) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecondition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecondition".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepriority(mut self, val: bool) -> Self {
        self.params.insert(
            "usepriority".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepriority".to_string(),
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
    pub fn with_overwriteexisting(mut self, val: bool) -> Self {
        self.params.insert(
            "overwriteexisting".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overwriteexisting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overwriteexisting".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keeporiginal(mut self, val: bool) -> Self {
        self.params.insert(
            "keeporiginal".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeporiginal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeporiginal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributerename {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributerename"
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
pub enum TopAttributestringeditExistingattrib {
    ReplaceExistingAttribute = 0,
    KeepExistingAttribute = 1,
    UpdateExistingAttribute = 2,
    /// Warn on Type Mis-match
    WarnOnTypeMisMinusMatch = 3,
    /// Error on Type Mis-match
    ErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone)]
pub struct TopAttributestringedit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopAttributestringedit {
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
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Int parameters ---
    pub fn with_enablewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "enablewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enablewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_index(mut self, val: i32) -> Self {
        self.params.insert(
            "index".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
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
    pub fn with_existingattrib(mut self, val: TopAttributestringeditExistingattrib) -> Self {
        self.params.insert(
            "existingattrib".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_existingattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "existingattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_from_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("from{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_from_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("from{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_to_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("to{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_to_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("to{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useenablewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "useenablewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useenablewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useenablewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useindex(mut self, val: bool) -> Self {
        self.params.insert(
            "useindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefilter_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("enablefilter{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefilter_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("enablefilter{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useregex_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useregex{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useregex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useregex{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_global_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("global{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_global_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("global{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopAttributestringedit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "attributestringedit"
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
