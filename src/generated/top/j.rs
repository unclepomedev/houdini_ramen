#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsoninputJsonsource {
    UpstreamOutputFile = 0,
    CustomFilePath = 1,
    Attribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsoninputOp {
    Retrieve = 0,
    ArrayRetrieve = 1,
    DeserializeWorkItem = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsoninputAttribcollision {
    KeepUpstreamAttribute = 0,
    KeepJsonAttribute = 1,
    ReportWarning = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsoninputKeyerrormode {
    AddWarning = 0,
    RaiseError = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsoninputAttributetype {
    Automatic = 0,
    String = 1,
    Integer = 2,
    Float = 3,
    Pyobject = 4,
    Dictionary = 5,
    StringArray = 6,
    IntegerArray = 7,
    FloatArray = 8,
    DictionaryArray = 9,
    UnpackedAttributes = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsoninputPreserveinput {
    None = 0,
    FileAttribute = 1,
    OutputFile = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsoninputValtype {
    Automatic = 0,
    String = 1,
    Integer = 2,
    Float = 3,
    Pyobject = 4,
    Dictionary = 5,
    StringArray = 6,
    IntegerArray = 7,
    FloatArray = 8,
    DictionaryArray = 9,
    UnpackedAttributes = 10,
}

#[derive(Debug, Clone)]
pub struct TopJsoninput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopJsoninput {
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

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_op(mut self, val: TopJsoninputOp) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keyerrormode(mut self, val: TopJsoninputKeyerrormode) -> Self {
        self.params.insert("keyerrormode".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_keyerrormode_expr(mut self, expr: &str) -> Self {
        self.params.insert("keyerrormode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_valindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("valindex{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_valindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("valindex{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitemgeneration".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemgeneration".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_jsonsource(mut self, val: TopJsoninputJsonsource) -> Self {
        self.params.insert("jsonsource".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_jsonsource_expr(mut self, expr: &str) -> Self {
        self.params.insert("jsonsource".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attribcollision(mut self, val: TopJsoninputAttribcollision) -> Self {
        self.params.insert("attribcollision".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_attribcollision_expr(mut self, expr: &str) -> Self {
        self.params.insert("attribcollision".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attributetype(mut self, val: TopJsoninputAttributetype) -> Self {
        self.params.insert("attributetype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_attributetype_expr(mut self, expr: &str) -> Self {
        self.params.insert("attributetype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preserveinput(mut self, val: TopJsoninputPreserveinput) -> Self {
        self.params.insert("preserveinput".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_preserveinput_expr(mut self, expr: &str) -> Self {
        self.params.insert("preserveinput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_valtype_inst(mut self, index1: usize, val: TopJsoninputValtype) -> Self {
        self.params.insert(format!("valtype{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_valtype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("valtype{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_filetag(mut self, val: &str) -> Self {
        self.params.insert("filetag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filetag_expr(mut self, expr: &str) -> Self {
        self.params.insert("filetag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_filepath(mut self, val: &str) -> Self {
        self.params.insert("filepath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filepath_expr(mut self, expr: &str) -> Self {
        self.params.insert("filepath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attribute(mut self, val: &str) -> Self {
        self.params.insert("attribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("attribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prop(mut self, val: &str) -> Self {
        self.params.insert("prop".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_prop_expr(mut self, expr: &str) -> Self {
        self.params.insert("prop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_field(mut self, val: &str) -> Self {
        self.params.insert("field".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_field_expr(mut self, expr: &str) -> Self {
        self.params.insert("field".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attributename(mut self, val: &str) -> Self {
        self.params.insert("attributename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attributename_expr(mut self, expr: &str) -> Self {
        self.params.insert("attributename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preserveinputattrib(mut self, val: &str) -> Self {
        self.params.insert("preserveinputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_preserveinputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("preserveinputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_query_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("query{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_query_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("query{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attributename_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("attributename{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attributename_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("attributename{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extracts(mut self, val: &str) -> Self {
        self.params.insert("extracts".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_extracts_expr(mut self, expr: &str) -> Self {
        self.params.insert("extracts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_preserveindex(mut self, val: bool) -> Self {
        self.params.insert("preserveindex".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_preserveindex_expr(mut self, expr: &str) -> Self {
        self.params.insert("preserveindex".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skipinvalid_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("skipinvalid{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_skipinvalid_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("skipinvalid{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skipinvalid(mut self, val: bool) -> Self {
        self.params.insert("skipinvalid".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_skipinvalid_expr(mut self, expr: &str) -> Self {
        self.params.insert("skipinvalid".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_splitarray(mut self, val: bool) -> Self {
        self.params.insert("splitarray".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_splitarray_expr(mut self, expr: &str) -> Self {
        self.params.insert("splitarray".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopJsoninput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "jsoninput"
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
pub enum TopJsonoutputOp {
    UpstreamWorkItemFields = 0,
    UpstreamOutputFile = 1,
    CustomFilePath = 2,
    Attribute = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopJsonoutputOutputtype {
    JsonFile = 0,
    StringAttribute = 1,
    PyobjectAttribute = 2,
    DictionaryAttribute = 3,
}

#[derive(Debug, Clone)]
pub struct TopJsonoutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopJsonoutput {
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

    /// Connects to input 0: "input"
    pub fn set_input_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_op(mut self, val: TopJsonoutputOp) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Int(val as i32));
        self
    }
    pub fn with_op_expr(mut self, expr: &str) -> Self {
        self.params.insert("op".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inputindex(mut self, val: i32) -> Self {
        self.params.insert("inputindex".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_inputindex_expr(mut self, expr: &str) -> Self {
        self.params.insert("inputindex".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitemgeneration".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemgeneration".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputtype(mut self, val: TopJsonoutputOutputtype) -> Self {
        self.params.insert("outputtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_outputtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_inputtag(mut self, val: &str) -> Self {
        self.params.insert("inputtag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_inputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert("inputtag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inputfile(mut self, val: &str) -> Self {
        self.params.insert("inputfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_inputfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("inputfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_inputattrib(mut self, val: &str) -> Self {
        self.params.insert("inputattrib".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_inputattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert("inputattrib".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputtag(mut self, val: &str) -> Self {
        self.params.insert("outputtag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputtag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputfile(mut self, val: &str) -> Self {
        self.params.insert("outputfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputattribute(mut self, val: &str) -> Self {
        self.params.insert("outputattribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_outputattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_writepartitions(mut self, val: bool) -> Self {
        self.params.insert("writepartitions".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_writepartitions_expr(mut self, expr: &str) -> Self {
        self.params.insert("writepartitions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_skipdefaults(mut self, val: bool) -> Self {
        self.params.insert("skipdefaults".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_skipdefaults_expr(mut self, expr: &str) -> Self {
        self.params.insert("skipdefaults".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputprettyprint(mut self, val: bool) -> Self {
        self.params.insert("outputprettyprint".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_outputprettyprint_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputprettyprint".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_outputsort(mut self, val: bool) -> Self {
        self.params.insert("outputsort".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_outputsort_expr(mut self, expr: &str) -> Self {
        self.params.insert("outputsort".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopJsonoutput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "jsonoutput"
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
