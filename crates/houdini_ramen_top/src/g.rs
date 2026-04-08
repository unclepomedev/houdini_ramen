#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGenericgeneratorUsedelay {
    CustomString = 0,
    Delay = 1,
    CommandBuilder = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGenericgeneratorValuetype {
    UpstreamOutputFiles = 0,
    AttributeName = 1,
    CustomValue = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGenericgeneratorCollapsearray {
    /// Space-Separated String
    SpaceMinusSeparatedString = 0,
    /// Comma-Separated String
    CommaMinusSeparatedString = 1,
    StringWithCustomSeparator = 2,
    MultipleArguments = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGenericgeneratorExpectedoutputsfrom {
    None = 0,
    Attribute = 1,
    FileList = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGenericgeneratorPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGenericgeneratorPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopGenericgenerator {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopGenericgenerator {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_frame(mut self, val: f32) -> Self {
        self.params.insert(
            "frame".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frame_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frame".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_delay(mut self, val: f32) -> Self {
        self.params.insert(
            "delay".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_delay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_itemcount(mut self, val: i32) -> Self {
        self.params.insert(
            "itemcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_itemcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "itemcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_index(mut self, val: i32) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_priority(mut self, val: i32) -> Self {
        self.params.insert(
            "priority".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "priority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cachemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedelay(mut self, val: TopGenericgeneratorUsedelay) -> Self {
        self.params.insert(
            "usedelay".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usedelay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedelay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_valuetype_inst(mut self, index1: usize, val: TopGenericgeneratorValuetype) -> Self {
        self.params.insert(
            format!("valuetype{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_valuetype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("valuetype{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collapsearray_inst(
        mut self,
        index1: usize,
        val: TopGenericgeneratorCollapsearray,
    ) -> Self {
        self.params.insert(
            format!("collapsearray{}", index1),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collapsearray_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collapsearray{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputsfrom(mut self, val: TopGenericgeneratorExpectedoutputsfrom) -> Self {
        self.params.insert(
            "expectedoutputsfrom".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_expectedoutputsfrom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputsfrom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopGenericgeneratorPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(
        mut self,
        val: TopGenericgeneratorPdgWorkitempriority,
    ) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_command(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_command_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_command".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_linuxcommand(mut self, val: &str) -> Self {
        self.params.insert(
            "linuxcommand".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_linuxcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "linuxcommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maccommand(mut self, val: &str) -> Self {
        self.params.insert(
            "maccommand".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maccommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maccommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_windowscommand(mut self, val: &str) -> Self {
        self.params.insert(
            "windowscommand".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_windowscommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "windowscommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basecommand(mut self, val: &str) -> Self {
        self.params.insert(
            "basecommand".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basecommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basecommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_argumentname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("argumentname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_argumentname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("argumentname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputtag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("inputtag{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("inputtag{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attribname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("attribname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attribname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("attribname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_argumentjoin_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("argumentjoin{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_argumentjoin_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("argumentjoin{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputtag{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputtag{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputattr(mut self, val: &str) -> Self {
        self.params.insert(
            "expectedoutputattr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputattr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputtag(mut self, val: &str) -> Self {
        self.params.insert(
            "expectedoutputtag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputtag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("expectedoutputfile{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("expectedoutputfile{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputtag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("expectedoutputtag{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expectedoutputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("expectedoutputtag{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pdgnodedep{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_sequential(mut self, val: bool) -> Self {
        self.params.insert(
            "sequential".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sequential_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sequential".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyinputs(mut self, val: bool) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_copyinputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_flattenindices(mut self, val: bool) -> Self {
        self.params.insert(
            "flattenindices".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_flattenindices_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flattenindices".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useindex(mut self, val: bool) -> Self {
        self.params.insert(
            "useindex".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useindex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useframe(mut self, val: bool) -> Self {
        self.params.insert(
            "useframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepriority(mut self, val: bool) -> Self {
        self.params.insert(
            "usepriority".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_platformcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "platformcommand".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_platformcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "platformcommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shellcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "shellcommand".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shellcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellcommand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_argumentenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("argumentenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_argumentenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("argumentenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_writeoutput_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("writeoutput{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writeoutput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("writeoutput{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ownoutput_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("ownoutput{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ownoutput_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("ownoutput{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useoutputtag_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useoutputtag{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useoutputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useoutputtag{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useexpectedoutputtag(mut self, val: bool) -> Self {
        self.params.insert(
            "useexpectedoutputtag".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexpectedoutputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useexpectedoutputtag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useexpectedoutputtag_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("useexpectedoutputtag{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexpectedoutputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("useexpectedoutputtag{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopGenericgenerator {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "genericgenerator"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportGeometrysource {
    SopNode = 0,
    /// Upstream Output File(s)
    UpstreamOutputFileS = 1,
    UpstreamGeometryAttribute = 2,
    CustomFilePath = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportUsetime {
    WorkItemFrame = 0,
    CustomTime = 1,
    NetworkEvaluationTime = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportEmptygeometry {
    ImportAsIs = 0,
    Ignore = 1,
    GenerateWarning = 2,
    GenerateError = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportMergeinput {
    ImportFirstGeometry = 0,
    ImportAndMergeAllGeometry = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportMergeclass {
    None = 0,
    Point = 1,
    Primitive = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportMergeattribtype {
    Integer = 0,
    String = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportStoragetype {
    None = 0,
    ExternalFile = 1,
    GeometryAttribute = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportIndextype {
    ElementIndex = 0,
    UpstreamItemIndex = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportClass {
    Point = 0,
    Vertex = 1,
    Primitive = 2,
    Detail = 3,
    BoundingBox = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportGrouptype {
    AllGroups = 0,
    SpecifiedGroup = 1,
    EachElementInSpecifiedGroup = 2,
    EachPieceAttributeValue = 3,
    EachElement = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopGeometryimportPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopGeometryimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopGeometryimport {
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

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_schedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemgeneration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geometrysource(mut self, val: TopGeometryimportGeometrysource) -> Self {
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
    pub fn with_usetime(mut self, val: TopGeometryimportUsetime) -> Self {
        self.params.insert(
            "usetime".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usetime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_emptygeometry(mut self, val: TopGeometryimportEmptygeometry) -> Self {
        self.params.insert(
            "emptygeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_emptygeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "emptygeometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mergeinput(mut self, val: TopGeometryimportMergeinput) -> Self {
        self.params.insert(
            "mergeinput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mergeinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mergeclass(mut self, val: TopGeometryimportMergeclass) -> Self {
        self.params.insert(
            "mergeclass".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mergeclass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeclass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mergeattribtype(mut self, val: TopGeometryimportMergeattribtype) -> Self {
        self.params.insert(
            "mergeattribtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mergeattribtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeattribtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_storagetype(mut self, val: TopGeometryimportStoragetype) -> Self {
        self.params.insert(
            "storagetype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_storagetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "storagetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indextype(mut self, val: TopGeometryimportIndextype) -> Self {
        self.params.insert(
            "indextype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_indextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indextype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_class(mut self, val: TopGeometryimportClass) -> Self {
        self.params.insert(
            "class".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_class_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "class".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_grouptype(mut self, val: TopGeometryimportGrouptype) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_grouptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "grouptype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addjobparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopGeometryimportPdgWorkitemlabel) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopGeometryimportPdgWorkitempriority) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
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
    pub fn with_filetag(mut self, val: &str) -> Self {
        self.params.insert(
            "filetag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filetag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filepath(mut self, val: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upstreamattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "upstreamattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_upstreamattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "upstreamattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mergeattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "mergeattribname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mergeattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeattribname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalpath(mut self, val: &str) -> Self {
        self.params.insert(
            "externalpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externaltag(mut self, val: &str) -> Self {
        self.params.insert(
            "externaltag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externaltag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externaltag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attributename(mut self, val: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attributename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attributes(mut self, val: &str) -> Self {
        self.params.insert(
            "attributes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_attributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributes".to_string(),
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
    pub fn with_pieceattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "pieceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pieceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pieceattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_workitemattributes(mut self, val: &str) -> Self {
        self.params.insert(
            "workitemattributes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_workitemattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "workitemattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_evaluateattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "evaluateattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_evaluateattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluateattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reloadfiles(mut self, val: bool) -> Self {
        self.params.insert(
            "reloadfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reloadfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reloadfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_asyncload(mut self, val: bool) -> Self {
        self.params.insert(
            "asyncload".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_asyncload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "asyncload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mergeexpandpartition(mut self, val: bool) -> Self {
        self.params.insert(
            "mergeexpandpartition".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mergeexpandpartition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mergeexpandpartition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_indexflatten(mut self, val: bool) -> Self {
        self.params.insert(
            "indexflatten".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indexflatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexflatten".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "copyattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_copyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copyattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_copybbox(mut self, val: bool) -> Self {
        self.params.insert(
            "copybbox".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_copybbox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "copybbox".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpoints(mut self, val: bool) -> Self {
        self.params.insert(
            "primpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_primpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpoints".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addworkitemattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "addworkitemattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addworkitemattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addworkitemattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_useschedulewhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopGeometryimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "geometryimport"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
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
