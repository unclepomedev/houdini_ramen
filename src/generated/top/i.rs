#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickPdgCooktype {
    /// In-Process
    InMinusProcess = 0,
    /// Out-of-Process
    OutMinusOfMinusProcess = 1,
    Service = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickPdgUseserviceblock {
    Never = 0,
    IfServiceNameMatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickBatchmode {
    Off = 0,
    AllItemsInOneBatch = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickCookwhen {
    AllItemsAreReady = 0,
    FirstItemIsReady = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickInputsource {
    UpstreamOutputFiles = 0,
    CustomFilePath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickOperation {
    Montage = 0,
    Convert = 1,
    Composite = 2,
    Compare = 3,
    Import = 4,
    Conjure = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickInputsort {
    None = 0,
    NaturalFilename = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickOrdermethod {
    NaturalFilename = 0,
    UpstreamIndex = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickOverlaymode {
    None = 0,
    InputFilename = 1,
    AttributeArray = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickArgumentpos {
    BeforeInput = 0,
    BeforeOutput = 1,
    AfterOutput = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickValuetype {
    AttributeName = 0,
    CustomValue = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickImagemagickbinary {
    SystemPath = 0,
    /// $PDG_IMAGEMAGICK_DIR
    PdgImagemagickDir = 1,
    /// $PDG_IMAGEMAGICK
    PdgImagemagick = 2,
    CustomPath = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopImagemagickPdgCommandtype {
    UseDefault = 0,
    CustomScript = 1,
    CustomCommand = 2,
}

#[derive(Debug, Clone)]
pub struct TopImagemagick {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopImagemagick {
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

    // --- Button parameters ---
    pub fn trigger_manageservices(mut self) -> Self {
        self.params.insert(
            "manageservices".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_operation(mut self, val: TopImagemagickOperation) -> Self {
        self.params.insert(
            "operation".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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
    pub fn with_ordermethod(mut self, val: TopImagemagickOrdermethod) -> Self {
        self.params.insert(
            "ordermethod".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_ordermethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ordermethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overlaymode(mut self, val: TopImagemagickOverlaymode) -> Self {
        self.params.insert(
            "overlaymode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_overlaymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overlaymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_montagecolumns(mut self, val: i32) -> Self {
        self.params.insert(
            "montagecolumns".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_montagecolumns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "montagecolumns".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_montagerows(mut self, val: i32) -> Self {
        self.params.insert(
            "montagerows".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_montagerows_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "montagerows".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imgwidth(mut self, val: i32) -> Self {
        self.params.insert(
            "imgwidth".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imgwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imgwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imgheight(mut self, val: i32) -> Self {
        self.params.insert(
            "imgheight".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imgheight_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imgheight".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imgquality(mut self, val: i32) -> Self {
        self.params.insert(
            "imgquality".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_imgquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imgquality".to_string(),
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
    pub fn with_pdg_cooktype(mut self, val: TopImagemagickPdgCooktype) -> Self {
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
    pub fn with_pdg_useserviceblock(mut self, val: TopImagemagickPdgUseserviceblock) -> Self {
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
    pub fn with_batchmode(mut self, val: TopImagemagickBatchmode) -> Self {
        self.params.insert(
            "batchmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_batchmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cookwhen(mut self, val: TopImagemagickCookwhen) -> Self {
        self.params.insert(
            "cookwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cookwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cookwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputsource(mut self, val: TopImagemagickInputsource) -> Self {
        self.params.insert(
            "inputsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputsort(mut self, val: TopImagemagickInputsort) -> Self {
        self.params.insert(
            "inputsort".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputsort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputsort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_argumentpos_inst(mut self, index1: usize, val: TopImagemagickArgumentpos) -> Self {
        self.params.insert(
            format!("argumentpos{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_argumentpos_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("argumentpos{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_valuetype_inst(mut self, index1: usize, val: TopImagemagickValuetype) -> Self {
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
    pub fn with_imagemagickbinary(mut self, val: TopImagemagickImagemagickbinary) -> Self {
        self.params.insert(
            "imagemagickbinary".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_imagemagickbinary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagemagickbinary".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopImagemagickPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopImagemagickPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_commandtype(mut self, val: TopImagemagickPdgCommandtype) -> Self {
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
    pub fn with_inputfiletag(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfiletag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfiletag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "outputfilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputfilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfiletag(mut self, val: &str) -> Self {
        self.params.insert(
            "outputfiletag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfiletag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputfiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
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
    pub fn with_overlayexpr(mut self, val: &str) -> Self {
        self.params.insert(
            "overlayexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_overlayexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overlayexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overlayattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "overlayattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_overlayattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overlayattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertfilename(mut self, val: &str) -> Self {
        self.params.insert(
            "convertfilename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_convertfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertfilename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compositeop(mut self, val: &str) -> Self {
        self.params.insert(
            "compositeop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_compositeop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compositeop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_background(mut self, val: &str) -> Self {
        self.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_background_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "background".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imageresolutions(mut self, val: &str) -> Self {
        self.params.insert(
            "imageresolutions".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_imageresolutions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imageresolutions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_argumentname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("argumentname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_argumentname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("argumentname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
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
    pub fn with_customvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customcommand(mut self, val: &str) -> Self {
        self.params.insert(
            "customcommand".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customcommand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customimagemagickbinary(mut self, val: &str) -> Self {
        self.params.insert(
            "customimagemagickbinary".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customimagemagickbinary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customimagemagickbinary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customarguments(mut self, val: &str) -> Self {
        self.params.insert(
            "customarguments".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customarguments_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customarguments".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagemagickbin(mut self, val: &str) -> Self {
        self.params.insert(
            "imagemagickbin".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_imagemagickbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagemagickbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_imagetag(mut self, val: &str) -> Self {
        self.params.insert(
            "imagetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_imagetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "imagetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filename(mut self, val: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filename".to_string(),
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
    pub fn with_enablemode(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_montagecolumnsswitch(mut self, val: bool) -> Self {
        self.params.insert(
            "montagecolumnsswitch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_montagecolumnsswitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "montagecolumnsswitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_montagerowsswitch(mut self, val: bool) -> Self {
        self.params.insert(
            "montagerowsswitch".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_montagerowsswitch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "montagerowsswitch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_splitresults(mut self, val: bool) -> Self {
        self.params.insert(
            "splitresults".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_splitresults_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitresults".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablebackground(mut self, val: bool) -> Self {
        self.params.insert(
            "enablebackground".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablebackground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablebackground".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updateresolution(mut self, val: bool) -> Self {
        self.params.insert(
            "updateresolution".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_updateresolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updateresolution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preservear(mut self, val: bool) -> Self {
        self.params.insert(
            "preservear".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preservear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preservear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableimgquality(mut self, val: bool) -> Self {
        self.params.insert(
            "enableimgquality".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableimgquality_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableimgquality".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecustomcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "usecustomcommand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustomcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecustomcommand".to_string(),
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

impl crate::core::types::HoudiniNode for TopImagemagick {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "imagemagick"
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
pub enum TopInprocessschedulerMaxworkitems {
    EqualToCpuCount = 0,
    CustomCount = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopInprocessschedulerTempdirmenu {
    WorkingDirectory = 0,
    HoudiniTemp = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopInprocessschedulerPdgDeletetempdir {
    Never = 0,
    WhenSchedulerIsDeleted = 1,
    WhenCookCompletes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopInprocessschedulerInprocessOnfailure {
    ReportError = 0,
    ReportWarning = 1,
    RetryTask = 2,
    Ignore = 3,
}

#[derive(Debug, Clone)]
pub struct TopInprocessscheduler {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopInprocessscheduler {
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
    pub fn with_customcount(mut self, val: i32) -> Self {
        self.params.insert(
            "customcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inprocess_maximumretries(mut self, val: i32) -> Self {
        self.params.insert(
            "inprocess_maximumretries".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_inprocess_maximumretries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocess_maximumretries".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_maxworkitems(mut self, val: TopInprocessschedulerMaxworkitems) -> Self {
        self.params.insert(
            "maxworkitems".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_maxworkitems_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxworkitems".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tempdirmenu(mut self, val: TopInprocessschedulerTempdirmenu) -> Self {
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
    pub fn with_pdg_deletetempdir(mut self, val: TopInprocessschedulerPdgDeletetempdir) -> Self {
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
    pub fn with_inprocess_onfailure(
        mut self,
        val: TopInprocessschedulerInprocessOnfailure,
    ) -> Self {
        self.params.insert(
            "inprocess_onfailure".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inprocess_onfailure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocess_onfailure".to_string(),
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
    pub fn with_inprocess_failcountattr(mut self, val: &str) -> Self {
        self.params.insert(
            "inprocess_failcountattr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inprocess_failcountattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocess_failcountattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
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
    pub fn with_inprocess_single(mut self, val: bool) -> Self {
        self.params.insert(
            "inprocess_single".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inprocess_single_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocess_single".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inprocess_mainthread(mut self, val: bool) -> Self {
        self.params.insert(
            "inprocess_mainthread".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inprocess_mainthread_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocess_mainthread".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inprocess_addfailcountattr(mut self, val: bool) -> Self {
        self.params.insert(
            "inprocess_addfailcountattr".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inprocess_addfailcountattr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocess_addfailcountattr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopInprocessscheduler {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "inprocessscheduler"
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
pub enum TopInvokeGeometrysource {
    UpstreamFile = 0,
    UpstreamGeometryData = 1,
    CustomFile = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopInvokeWriteoutput {
    Attribute = 0,
    File = 1,
    Both = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopInvokePdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopInvokePdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopInvoke {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopInvoke {
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
    pub fn with_geometrysource_inst(mut self, index1: usize, val: TopInvokeGeometrysource) -> Self {
        self.params.insert(
            format!("geometrysource{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geometrysource_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("geometrysource{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_writeoutput(mut self, val: TopInvokeWriteoutput) -> Self {
        self.params.insert(
            "writeoutput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writeoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writeoutput".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopInvokePdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopInvokePdgWorkitempriority) -> Self {
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
    pub fn with_attribute_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("attribute{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attribute_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("attribute{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filepath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filepath{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_filepath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filepath{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("inputname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("inputname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "outputattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputpath(mut self, val: &str) -> Self {
        self.params.insert(
            "outputpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtag(mut self, val: &str) -> Self {
        self.params.insert(
            "outputtag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtag".to_string(),
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
    pub fn with_usetime(mut self, val: bool) -> Self {
        self.params.insert(
            "usetime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
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
    pub fn with_expandpartitions(mut self, val: bool) -> Self {
        self.params.insert(
            "expandpartitions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expandpartitions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expandpartitions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loadduringcook(mut self, val: bool) -> Self {
        self.params.insert(
            "loadduringcook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loadduringcook_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loadduringcook".to_string(),
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

impl crate::core::types::HoudiniNode for TopInvoke {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "invoke"
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
