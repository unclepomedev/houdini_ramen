#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTextoutputWriteduring {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTextoutputPdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTextoutputAppend {
    Overwrite = 0,
    Append = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTextoutputPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTextoutputPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopTextoutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopTextoutput {
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
    pub fn with_writeduring(mut self, val: TopTextoutputWriteduring) -> Self {
        self.params.insert(
            "writeduring".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writeduring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writeduring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopTextoutputPdgCooktype) -> Self {
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
    pub fn with_append(mut self, val: TopTextoutputAppend) -> Self {
        self.params.insert(
            "append".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_append_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "append".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopTextoutputPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopTextoutputPdgWorkitempriority) -> Self {
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
    pub fn with_text(mut self, val: &str) -> Self {
        self.params.insert(
            "text".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_text_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "text".to_string(),
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

impl crate::core::types::HoudiniNode for TopTextoutput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "textoutput"
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
pub enum TopTexttocsvParseMethod {
    OneColumnOnly = 0,
    LexicalSplit = 1,
    CreateColumnsFromDelimiter = 2,
}

#[derive(Debug, Clone)]
pub struct TopTexttocsv {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopTexttocsv {
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
    pub fn with_parse_method(mut self, val: TopTexttocsvParseMethod) -> Self {
        self.params.insert(
            "parse_method".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_parse_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parse_method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_remove_lines(mut self, val: &str) -> Self {
        self.params.insert(
            "remove_lines".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_remove_lines_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remove_lines".to_string(),
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
    pub fn with_output_filename(mut self, val: &str) -> Self {
        self.params.insert(
            "output_filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_output_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "output_filename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_remove_empty_col(mut self, val: bool) -> Self {
        self.params.insert(
            "remove_empty_col".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_remove_empty_col_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remove_empty_col".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopTexttocsv {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "texttocsv"
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
pub enum TopTopfetchCooktype {
    OutputNode = 0,
    OutputWorkItems = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTopfetchVerbosity {
    NoLogging = 0,
    ErrorsOnly = 1,
    ErrorsAndInfo = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTopfetchPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTopfetchPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopTopfetch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopTopfetch {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
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
    pub fn with_cooktype(mut self, val: TopTopfetchCooktype) -> Self {
        self.params.insert(
            "cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbosity(mut self, val: TopTopfetchVerbosity) -> Self {
        self.params.insert(
            "verbosity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_verbosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbosity".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopTopfetchPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopTopfetchPdgWorkitempriority) -> Self {
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
    pub fn with_hipname(mut self, val: &str) -> Self {
        self.params.insert(
            "hipname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hipname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputdir(mut self, val: &str) -> Self {
        self.params.insert(
            "outputdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputdir".to_string(),
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
    pub fn with_reportall(mut self, val: bool) -> Self {
        self.params.insert(
            "reportall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableoutputdir(mut self, val: bool) -> Self {
        self.params.insert(
            "enableoutputdir".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableoutputdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableoutputdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_printlogs(mut self, val: bool) -> Self {
        self.params.insert(
            "printlogs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_printlogs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "printlogs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sequential(mut self, val: bool) -> Self {
        self.params.insert(
            "sequential".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sequential_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sequential".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expand(mut self, val: bool) -> Self {
        self.params.insert(
            "expand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_keepfailed(mut self, val: bool) -> Self {
        self.params.insert(
            "keepfailed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepfailed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepfailed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preserveindices(mut self, val: bool) -> Self {
        self.params.insert(
            "preserveindices".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preserveindices_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preserveindices".to_string(),
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

impl crate::core::types::HoudiniNode for TopTopfetch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "topfetch"
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
pub enum TopTopfetchinputWorkitemsource {
    JsonFile = 0,
    ActiveWorkItem = 1,
    Automatic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTopfetchinputMissinginput {
    ReportError = 0,
    ReportWarning = 1,
    GenerateEmptyWorkItems = 2,
}

#[derive(Debug, Clone)]
pub struct TopTopfetchinput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopTopfetchinput {
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
    pub fn with_missingcount(mut self, val: i32) -> Self {
        self.params.insert(
            "missingcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_missingcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missingcount".to_string(),
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
    pub fn with_workitemsource(mut self, val: TopTopfetchinputWorkitemsource) -> Self {
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
    pub fn with_missinginput(mut self, val: TopTopfetchinputMissinginput) -> Self {
        self.params.insert(
            "missinginput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missinginput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missinginput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_jsonfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "jsonfilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_jsonfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jsonfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopTopfetchinput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "topfetchinput"
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
pub enum TopTopnetCheckpointformat {
    /// Python (Deprecated)
    PythonDeprecated = 0,
    Json = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTopnetCheckpointload {
    Never = 0,
    OnSceneLoad = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTopnetRegenerationtype {
    UpdateWorkItemsAndInvalidateCaches = 0,
    UpdateWorkItemsOnly = 1,
    IgnoreChanges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTopnetEvaluationtime {
    NetworkCookTime = 0,
    GlobalStartTime = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct TopTopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopTopnet {
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
    pub fn trigger_generatestatic(mut self) -> Self {
        self.params.insert(
            "generatestatic".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookbutton(mut self) -> Self {
        self.params.insert(
            "cookbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtybutton(mut self) -> Self {
        self.params.insert(
            "dirtybutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cancelbutton(mut self) -> Self {
        self.params.insert(
            "cancelbutton".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savetaskgraph(mut self) -> Self {
        self.params.insert(
            "savetaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadtaskgraph(mut self) -> Self {
        self.params.insert(
            "loadtaskgraph".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadcheckpoint(mut self) -> Self {
        self.params.insert(
            "loadcheckpoint".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_taskgraphsaverate(mut self, val: i32) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskgraphsaverate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointrate(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customtime(mut self, val: i32) -> Self {
        self.params.insert(
            "customtime".to_string(),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_checkpointformat(mut self, val: TopTopnetCheckpointformat) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointload(mut self, val: TopTopnetCheckpointload) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_regenerationtype(mut self, val: TopTopnetRegenerationtype) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_regenerationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_evaluationtime(mut self, val: TopTopnetEvaluationtime) -> Self {
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

    // --- String parameters ---
    pub fn with_taskgraphfile(mut self, val: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_taskgraphfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointfile(mut self, val: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_checkpointfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
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
    pub fn with_defaultlabel(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_defaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_taskgraphautosave(mut self, val: bool) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_taskgraphautosave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpointenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savegraphattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savegraphattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedefaultlabel(mut self, val: bool) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedefaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savescenefile(mut self, val: bool) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savescenefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopTopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "topnet"
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
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait TopTopnetInnerExt {
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> TopTopnetInnerExt for crate::core::graph::InnerGraph<'a> {
    fn localscheduler(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("localscheduler")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerHythonbin {
    Default = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerPdgWorkitemdatasource {
    TemporaryJsonFile = 0,
    RpcMessage = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerPdgMapmode {
    Global = 0,
    None = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerPdgTransfertype {
    FlatCopy = 0,
    RelativeToRoot = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerTempdirmenu {
    WorkingDirectory = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerPdgDeletetempdir {
    Never = 0,
    WhenSchedulerIsDeleted = 1,
    WhenCookCompletes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerSubmitjobverbosity {
    None = 0,
    ErrorsOnly = 1,
    Full = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerUsedatalayerport {
    Automatic = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerSubmitjobwhenfinished {
    Terminate = 0,
    KeepRunningIfError = 1,
    KeepRunning = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerPdgRpcignoreerrors {
    Never = 0,
    WhenCookingBatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopTractorschedulerTractorEchandleby {
    ReportingError = 0,
    ReportingWarning = 1,
    RetryingTask = 2,
    IgnoringExitCode = 3,
}

#[derive(Debug, Clone)]
pub struct TopTractorscheduler {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopTractorscheduler {
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
    pub fn trigger_submitjob(mut self) -> Self {
        self.params.insert(
            "submitjob".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_tractor_priority(mut self, val: f32) -> Self {
        self.params.insert(
            "tractor_priority".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tractor_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_priority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcbatch(mut self, val: f32) -> Self {
        self.params.insert(
            "pdg_rpcbatch".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdg_rpcbatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcbatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_postsuccesswait(mut self, val: f32) -> Self {
        self.params.insert(
            "tractor_postsuccesswait".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tractor_postsuccesswait_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_postsuccesswait".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_tractor_port(mut self, val: i32) -> Self {
        self.params.insert(
            "tractor_port".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tractor_port_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_port".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_maxactive(mut self, val: i32) -> Self {
        self.params.insert(
            "maxactive".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxactive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxactive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datalayerserverport(mut self, val: i32) -> Self {
        self.params.insert(
            "datalayerserverport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_datalayerserverport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datalayerserverport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taskcallbackport(mut self, val: i32) -> Self {
        self.params.insert(
            "taskcallbackport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskcallbackport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskcallbackport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqrelayport(mut self, val: i32) -> Self {
        self.params.insert(
            "mqrelayport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mqrelayport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqrelayport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpctimeout(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpctimeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpctimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpctimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcretries(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcretries".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcretries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcretries".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcbackoff(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcbackoff".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcbackoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcbackoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_atleast(mut self, val: i32) -> Self {
        self.params.insert(
            "tractor_atleast".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tractor_atleast_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_atleast".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_atmost(mut self, val: i32) -> Self {
        self.params.insert(
            "tractor_atmost".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tractor_atmost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_atmost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_maxrunsecs(mut self, val: i32) -> Self {
        self.params.insert(
            "tractor_maxrunsecs".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tractor_maxrunsecs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_maxrunsecs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_eccustomcode(mut self, val: i32) -> Self {
        self.params.insert(
            "tractor_eccustomcode".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tractor_eccustomcode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_eccustomcode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_triesleft(mut self, val: i32) -> Self {
        self.params.insert(
            "tractor_triesleft".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tractor_triesleft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_triesleft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_houdinimaxthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "tractor_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_tractor_houdinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_hythonbin(mut self, val: TopTractorschedulerHythonbin) -> Self {
        self.params.insert(
            "hythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemdatasource(
        mut self,
        val: TopTractorschedulerPdgWorkitemdatasource,
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
    pub fn with_pdg_mapmode(mut self, val: TopTractorschedulerPdgMapmode) -> Self {
        self.params.insert(
            "pdg_mapmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_mapmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_mapmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_transfertype(mut self, val: TopTractorschedulerPdgTransfertype) -> Self {
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
    pub fn with_tempdirmenu(mut self, val: TopTractorschedulerTempdirmenu) -> Self {
        self.params.insert(
            "tempdirmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tempdirmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tempdirmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_deletetempdir(mut self, val: TopTractorschedulerPdgDeletetempdir) -> Self {
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
    pub fn with_submitjobverbosity(mut self, val: TopTractorschedulerSubmitjobverbosity) -> Self {
        self.params.insert(
            "submitjobverbosity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobverbosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobverbosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedatalayerport(mut self, val: TopTractorschedulerUsedatalayerport) -> Self {
        self.params.insert(
            "usedatalayerport".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usedatalayerport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedatalayerport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobwhenfinished(
        mut self,
        val: TopTractorschedulerSubmitjobwhenfinished,
    ) -> Self {
        self.params.insert(
            "submitjobwhenfinished".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobwhenfinished_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobwhenfinished".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors(mut self, val: TopTractorschedulerPdgRpcignoreerrors) -> Self {
        self.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_echandleby(mut self, val: TopTractorschedulerTractorEchandleby) -> Self {
        self.params.insert(
            "tractor_echandleby".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_tractor_echandleby_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_echandleby".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_tractor_hostname(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_hostname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_hostname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_hostname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_user(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_user".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_user_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_user".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_pass(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_pass".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_pass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_pass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_pythonexe(mut self, val: &str) -> Self {
        self.params.insert(
            "pythonexe".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pythonexe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonexe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hythonbincustomuniversal(mut self, val: &str) -> Self {
        self.params.insert(
            "hythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hythonbincustomuniversal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_mapzone(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_mapzone".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_mapzone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_mapzone".to_string(),
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
    pub fn with_nfsremotesharedroot(mut self, val: &str) -> Self {
        self.params.insert(
            "nfsremotesharedroot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nfsremotesharedroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nfsremotesharedroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uncremotesharedroot(mut self, val: &str) -> Self {
        self.params.insert(
            "uncremotesharedroot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uncremotesharedroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uncremotesharedroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_nfshfs(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_nfshfs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_nfshfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_nfshfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_unchfs(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_unchfs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_unchfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_unchfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tempdircustom(mut self, val: &str) -> Self {
        self.params.insert(
            "tempdircustom".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tempdircustom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tempdircustom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jobowner(mut self, val: &str) -> Self {
        self.params.insert(
            "jobowner".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_jobowner_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jobowner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jobname(mut self, val: &str) -> Self {
        self.params.insert(
            "jobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_jobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_tier(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_tier".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_tier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_tier".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_projects(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_projects".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_projects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_projects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_afterjids(mut self, val: &str) -> Self {
        self.params.insert(
            "afterjids".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_afterjids_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "afterjids".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobname(mut self, val: &str) -> Self {
        self.params.insert(
            "submitjobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitjobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobnode(mut self, val: &str) -> Self {
        self.params.insert(
            "submitjobnode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitjobnode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobnode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitasjobservicekey(mut self, val: &str) -> Self {
        self.params.insert(
            "submitasjobservicekey".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitasjobservicekey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitasjobservicekey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remotegraphname(mut self, val: &str) -> Self {
        self.params.insert(
            "remotegraphname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_remotegraphname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remotegraphname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqservicekey(mut self, val: &str) -> Self {
        self.params.insert(
            "mqservicekey".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mqservicekey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqservicekey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_servicekeys(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_servicekeys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_servicekeys_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_servicekeys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_limittags(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_limittags".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_limittags_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_limittags".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_envkeys(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_envkeys".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_envkeys_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_envkeys".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_tasktitle(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_tasktitle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_tasktitle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_tasktitle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_metadata(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_metadata".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_metadata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_metadata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_preview(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_preview".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_preview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_preview".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_envunset(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_envunset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_envunset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_envunset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_env_file(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_env_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_env_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_env_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_envname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("tractor_envname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_envname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("tractor_envname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_envvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("tractor_envvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_envvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("tractor_envvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_preshell(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_preshell".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_preshell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_preshell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_postshell(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_postshell".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_postshell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_postshell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_prepy(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_prepy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_prepy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_prepy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_postpy(mut self, val: &str) -> Self {
        self.params.insert(
            "tractor_postpy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tractor_postpy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_postpy".to_string(),
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
    pub fn with_autoretryfailedtask(mut self, val: bool) -> Self {
        self.params.insert(
            "autoretryfailedtask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoretryfailedtask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoretryfailedtask".to_string(),
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
    pub fn with_pdg_usemapzone(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_usemapzone".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemapzone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_usemapzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tempdirappendpid(mut self, val: bool) -> Self {
        self.params.insert(
            "tempdirappendpid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tempdirappendpid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tempdirappendpid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxactive(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaxactive".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxactive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaxactive".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesessionfile(mut self, val: bool) -> Self {
        self.params.insert(
            "usesessionfile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesessionfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesessionfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesubmitjobnode(mut self, val: bool) -> Self {
        self.params.insert(
            "usesubmitjobnode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesubmitjobnode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesubmitjobnode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobfile(mut self, val: bool) -> Self {
        self.params.insert(
            "submitjobfile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_submitjobfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledatalayerserver(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledatalayerserver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledatalayerserver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledatalayerserver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createremotegraph(mut self, val: bool) -> Self {
        self.params.insert(
            "createremotegraph".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createremotegraph_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createremotegraph".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetaskcallbackport(mut self, val: bool) -> Self {
        self.params.insert(
            "usetaskcallbackport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetaskcallbackport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetaskcallbackport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemqrelayport(mut self, val: bool) -> Self {
        self.params.insert(
            "usemqrelayport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemqrelayport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemqrelayport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_debug(mut self, val: bool) -> Self {
        self.params.insert(
            "tractor_debug".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tractor_debug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_debug".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcrelease(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_rpcrelease".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_rpcrelease_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcrelease".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_useatmost(mut self, val: bool) -> Self {
        self.params.insert(
            "tractor_useatmost".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tractor_useatmost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_useatmost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_echandleall(mut self, val: bool) -> Self {
        self.params.insert(
            "tractor_echandleall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tractor_echandleall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_echandleall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_inheritlocalenv(mut self, val: bool) -> Self {
        self.params.insert(
            "tractor_inheritlocalenv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tractor_inheritlocalenv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_inheritlocalenv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_usehoudinimaxthreads(mut self, val: bool) -> Self {
        self.params.insert(
            "tractor_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tractor_usehoudinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopTractorscheduler {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "tractorscheduler"
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
