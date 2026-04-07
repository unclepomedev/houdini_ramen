#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorOperatorselection {
    FirstDefinitionInAsset = 0,
    CustomString = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorAddhdaparms {
    Automatically = 0,
    Manually = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorPdgCooktype {
    /// In-Process
    InMinusProcess = 0,
    /// Out-of-Process
    OutMinusOfMinusProcess = 1,
    Service = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorBatchmode {
    Off = 0,
    AllItemsInOneBatch = 1,
    CustomBatchSize = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorCookwhen {
    AllItemsAreReady = 0,
    FirstItemIsReady = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorInputsource {
    UpstreamOutputFiles = 0,
    CustomFilePaths = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorMissinginput {
    RaiseError = 0,
    Ignore = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHdaprocessorPdgCommandtype {
    UseDefault = 0,
    CustomScript = 1,
    CustomCommand = 2,
}

#[derive(Debug, Clone)]
pub struct TopHdaprocessor {
    pub base: crate::core::types::NodeBase,
}

impl TopHdaprocessor {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_updatehdaparms(mut self) -> Self {
        self.base.params.insert(
            "updateHDAParms".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_filterparms(mut self) -> Self {
        self.base.params.insert(
            "filterparms".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_manageservices(mut self) -> Self {
        self.base.params.insert(
            "manageservices".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_serviceport(mut self, val: i32) -> Self {
        self.base.params.insert(
            "serviceport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_serviceport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "serviceport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_connectiontimeout(mut self, val: i32) -> Self {
        self.base.params.insert(
            "connectiontimeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_connectiontimeout_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "connectiontimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_itempriority(mut self, val: i32) -> Self {
        self.base.params.insert(
            "itempriority".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_itempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "itempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchcount(mut self, val: i32) -> Self {
        self.base.params.insert(
            "batchcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_batchcount_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "batchcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_operatorselection(mut self, val: TopHdaprocessorOperatorselection) -> Self {
        self.base.params.insert(
            "operatorselection".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operatorselection_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "operatorselection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addhdaparms(mut self, val: TopHdaprocessorAddhdaparms) -> Self {
        self.base.params.insert(
            "addhdaparms".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addhdaparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addhdaparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cachemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopHdaprocessorPdgCooktype) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchmode(mut self, val: TopHdaprocessorBatchmode) -> Self {
        self.base.params.insert(
            "batchmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_batchmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "batchmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cookwhen(mut self, val: TopHdaprocessorCookwhen) -> Self {
        self.base.params.insert(
            "cookwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cookwhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cookwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputsource(mut self, val: TopHdaprocessorInputsource) -> Self {
        self.base.params.insert(
            "inputsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputsource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_missinginput(mut self, val: TopHdaprocessorMissinginput) -> Self {
        self.base.params.insert(
            "missinginput".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missinginput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "missinginput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hdatype(mut self, val: i32) -> Self {
        self.base.params.insert(
            "hdatype".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_hdatype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hdatype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopHdaprocessorPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopHdaprocessorPdgWorkitempriority) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_commandtype(mut self, val: TopHdaprocessorPdgCommandtype) -> Self {
        self.base.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_commandtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_commandtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_displayedparms(mut self, val: &str) -> Self {
        self.base.params.insert(
            "displayedparms".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_displayedparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "displayedparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expressionparms(mut self, val: &str) -> Self {
        self.base.params.insert(
            "expressionparms".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expressionparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "expressionparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_serviceaddress(mut self, val: &str) -> Self {
        self.base.params.insert(
            "serviceaddress".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_serviceaddress_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "serviceaddress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_templatenode(mut self, val: &str) -> Self {
        self.base.params.insert(
            "templatenode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_templatenode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "templatenode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_operatortype(mut self, val: &str) -> Self {
        self.base.params.insert(
            "operatortype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_operatortype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "operatortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdgservicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdgservicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgservicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdgservicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inputfiletag(mut self, val: &str) -> Self {
        self.base.params.insert(
            "inputfiletag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_inputfiletag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inputfiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("customfile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("customfile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hdasopname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hdasopname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hdasopname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hdasopname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("outputfile{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("outputfile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtag_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("outputtag{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputtag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("outputtag{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputtag(mut self, val: &str) -> Self {
        self.base.params.insert(
            "outputtag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputtag_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "outputtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_perffile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "perffile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_perffile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "perffile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debughipfile(mut self, val: &str) -> Self {
        self.base.params.insert(
            "debughipfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_debughipfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debughipfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_command(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_command_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_command".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usehdaservice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usehdaservice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usehdaservice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usehdaservice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_autoexpression(mut self, val: bool) -> Self {
        self.base.params.insert(
            "autoexpression".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoexpression_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "autoexpression".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_generatefromitems(mut self, val: bool) -> Self {
        self.base.params.insert(
            "generatefromitems".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_generatefromitems_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "generatefromitems".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepriority(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usepriority".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usepriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablepdgservice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enablepdgservice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepdgservice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enablepdgservice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inprocesscleanup(mut self, val: bool) -> Self {
        self.base.params.insert(
            "inprocesscleanup".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inprocesscleanup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "inprocesscleanup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sequential(mut self, val: bool) -> Self {
        self.base.params.insert(
            "sequential".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sequential_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sequential".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfileinput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createfileinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfileinput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createfileinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecustomfile_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("usecustomfile{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustomfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("usecustomfile{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_writeoutput(mut self, val: bool) -> Self {
        self.base.params.insert(
            "writeoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writeoutput_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writeoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputenabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("outputenabled{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputenabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("outputenabled{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_writegeo(mut self, val: bool) -> Self {
        self.base.params.insert(
            "writegeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writegeo_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writegeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableperfmon(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enableperfmon".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableperfmon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enableperfmon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_saveperffile(mut self, val: bool) -> Self {
        self.base.params.insert(
            "saveperffile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_saveperffile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "saveperffile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dumpdebug(mut self, val: bool) -> Self {
        self.base.params.insert(
            "dumpdebug".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dumpdebug_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dumpdebug".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debugoutputs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "debugoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_debugoutputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "debugoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reportnestederrors(mut self, val: bool) -> Self {
        self.base.params.insert(
            "reportnestederrors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnestederrors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "reportnestederrors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopHdaprocessor {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "hdaprocessor"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHoudiniserverCopyinputs {
    NoIterations = 0,
    FirstIteration = 1,
    AllIterations = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHoudiniserverPdgCooktype {
    /// Shared Server (Deprecated)
    SharedServerDeprecated = 0,
    Service = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHoudiniserverPdgServicereset {
    None = 0,
    ResetClient = 1,
    RestartClient = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHoudiniserverPdgServiceresetwhen {
    BeforeCook = 0,
    AfterCook = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHoudiniserverPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHoudiniserverPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopHoudiniserver {
    pub base: crate::core::types::NodeBase,
}

impl TopHoudiniserver {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_manageservices(mut self) -> Self {
        self.base.params.insert(
            "manageservices".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_commandport(mut self, val: i32) -> Self {
        self.base.params.insert(
            "commandport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_commandport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "commandport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeout(mut self, val: i32) -> Self {
        self.base.params.insert(
            "timeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timeout_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_schedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriorityexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemgeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_copyinputs(mut self, val: TopHoudiniserverCopyinputs) -> Self {
        self.base.params.insert(
            "copyinputs".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_copyinputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "copyinputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopHoudiniserverPdgCooktype) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_cooktype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicereset(mut self, val: TopHoudiniserverPdgServicereset) -> Self {
        self.base.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_servicereset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_servicereset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen(mut self, val: TopHoudiniserverPdgServiceresetwhen) -> Self {
        self.base.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_serviceresetwhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "addjobparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopHoudiniserverPdgWorkitemlabel) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopHoudiniserverPdgWorkitempriority) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitempriority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_pdg_feedbackpattern(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackpattern_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalhost(mut self, val: &str) -> Self {
        self.base.params.insert(
            "externalhost".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalhost_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalhost".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_serverbinary(mut self, val: &str) -> Self {
        self.base.params.insert(
            "serverbinary".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_serverbinary_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "serverbinary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_servicename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_iterattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "iterattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_iterattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sizeattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "sizeattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sizeattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "sizeattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "numattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_numattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "numattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("pdgnodedep{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "topscheduler".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemlabelexpr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_iterationsfromupstream(mut self, val: bool) -> Self {
        self.base.params.insert(
            "iterationsfromupstream".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iterationsfromupstream_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterationsfromupstream".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loopsequential(mut self, val: bool) -> Self {
        self.base.params.insert(
            "loopsequential".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loopsequential_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "loopsequential".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackattribs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_feedbackattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackattribs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_feedbackfiles(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_feedbackfiles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackfiles_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_feedbackfiles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalserver(mut self, val: bool) -> Self {
        self.base.params.insert(
            "externalserver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_externalserver_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "externalserver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_useschedulewhen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopHoudiniserver {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "houdiniserver"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerPdgWorkitemdatasource {
    TemporaryJsonFile = 0,
    RpcMessage = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerPdgDeletetempdir {
    Never = 0,
    WhenSchedulerIsDeleted = 1,
    WhenCookCompletes = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerPdgMapmode {
    Global = 0,
    None = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerPdgTransfertype {
    FlatCopy = 0,
    RelativeToRoot = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerPythonbin {
    FromHfs = 0,
    Default = 1,
    Custom = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerHythonbin {
    Default = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerSubmitjobverbosity {
    None = 0,
    ErrorsOnly = 1,
    Full = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerSubmitjobassign {
    AnyClient = 0,
    ListedClients = 1,
    ClientsFromListedGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerUsedatalayerport {
    Automatic = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerSubmitjobwhenfinished {
    Terminate = 0,
    KeepRunningIfError = 1,
    KeepRunning = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerMqusage {
    Farm = 0,
    Local = 1,
    Connect = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerPdgRpcignoreerrors {
    Never = 0,
    WhenCookingBatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerHqueuePriority {
    /// 0 (lowest)
    N0Lowest = 0,
    /// 1
    N1 = 1,
    /// 2
    N2 = 2,
    /// 3
    N3 = 3,
    /// 4
    N4 = 4,
    /// 5 (medium)
    N5Medium = 5,
    /// 6
    N6 = 6,
    /// 7
    N7 = 7,
    /// 8
    N8 = 8,
    /// 9
    N9 = 9,
    /// 10 (highest)
    N10Highest = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerHqueueAssignTo {
    AnyClient = 0,
    ListedClients = 1,
    ClientsFromListedGroups = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerHqueueCreatecontainerjob {
    None = 0,
    FromNodeName = 1,
    FromCustomName = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopHqueueschedulerHqueueEchandleby {
    ReportingError = 0,
    ReportingWarning = 1,
    RetryingTask = 2,
    IgnoringExitCode = 3,
}

#[derive(Debug, Clone)]
pub struct TopHqueuescheduler {
    pub base: crate::core::types::NodeBase,
}

impl TopHqueuescheduler {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_loadpathmap(mut self) -> Self {
        self.base.params.insert(
            "loadpathmap".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_queryhq(mut self) -> Self {
        self.base.params.insert(
            "queryhq".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_submitjob(mut self) -> Self {
        self.base.params.insert(
            "submitjob".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_submitjobselectclients(mut self) -> Self {
        self.base.params.insert(
            "submitjobselectclients".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_submitjobselectgroups(mut self) -> Self {
        self.base.params.insert(
            "submitjobselectgroups".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_clients(mut self) -> Self {
        self.base.params.insert(
            "select_clients".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_client_groups(mut self) -> Self {
        self.base.params.insert(
            "select_client_groups".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_pdg_tickperiod(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pdg_tickperiod".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdg_tickperiod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_tickperiod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcbatch(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pdg_rpcbatch".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdg_rpcbatch_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_rpcbatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_pdg_maxtasks(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_maxtasks".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_maxtasks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_maxtasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_maxitems(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_maxitems".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_maxitems_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_maxitems".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobcpus(mut self, val: i32) -> Self {
        self.base.params.insert(
            "submitjobcpus".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_submitjobcpus_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobcpus".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_datalayerserverport(mut self, val: i32) -> Self {
        self.base.params.insert(
            "datalayerserverport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_datalayerserverport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "datalayerserverport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_taskcallbackport(mut self, val: i32) -> Self {
        self.base.params.insert(
            "taskcallbackport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskcallbackport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "taskcallbackport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqrelayport(mut self, val: i32) -> Self {
        self.base.params.insert(
            "mqrelayport".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mqrelayport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mqrelayport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpctimeout(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_rpctimeout".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpctimeout_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_rpctimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcretries(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_rpcretries".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcretries_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_rpcretries".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcbackoff(mut self, val: i32) -> Self {
        self.base.params.insert(
            "pdg_rpcbackoff".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcbackoff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_rpcbackoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_cpus_to_use(mut self, val: i32) -> Self {
        self.base.params.insert(
            "hqueue_CPUs_to_use".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_cpus_to_use_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_CPUs_to_use".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_houdinimaxthreads(mut self, val: i32) -> Self {
        self.base.params.insert(
            "hqueue_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_houdinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_houdinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_maxtime(mut self, val: i32) -> Self {
        self.base.params.insert(
            "hqueue_maxtime".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_maxtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_maxtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_resqty_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("hqueue_resqty{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_resqty_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("hqueue_resqty{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_eccustomcode(mut self, val: i32) -> Self {
        self.base.params.insert(
            "hqueue_eccustomcode".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_eccustomcode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_eccustomcode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_triesleft(mut self, val: i32) -> Self {
        self.base.params.insert(
            "hqueue_triesleft".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_triesleft_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_triesleft".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_workitemdatasource(
        mut self,
        val: TopHqueueschedulerPdgWorkitemdatasource,
    ) -> Self {
        self.base.params.insert(
            "pdg_workitemdatasource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemdatasource_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workitemdatasource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_deletetempdir(mut self, val: TopHqueueschedulerPdgDeletetempdir) -> Self {
        self.base.params.insert(
            "pdg_deletetempdir".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_deletetempdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_deletetempdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_mapmode(mut self, val: TopHqueueschedulerPdgMapmode) -> Self {
        self.base.params.insert(
            "pdg_mapmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_mapmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_mapmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_transfertype(mut self, val: TopHqueueschedulerPdgTransfertype) -> Self {
        self.base.params.insert(
            "pdg_transfertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_transfertype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_transfertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopHqueueschedulerPythonbin) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hythonbin(mut self, val: TopHqueueschedulerHythonbin) -> Self {
        self.base.params.insert(
            "hythonbin".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hythonbin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hythonbin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobverbosity(mut self, val: TopHqueueschedulerSubmitjobverbosity) -> Self {
        self.base.params.insert(
            "submitjobverbosity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobverbosity_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobverbosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobassign(mut self, val: TopHqueueschedulerSubmitjobassign) -> Self {
        self.base.params.insert(
            "submitjobassign".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobassign_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobassign".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedatalayerport(mut self, val: TopHqueueschedulerUsedatalayerport) -> Self {
        self.base.params.insert(
            "usedatalayerport".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usedatalayerport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usedatalayerport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobwhenfinished(
        mut self,
        val: TopHqueueschedulerSubmitjobwhenfinished,
    ) -> Self {
        self.base.params.insert(
            "submitjobwhenfinished".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobwhenfinished_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobwhenfinished".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqusage(mut self, val: TopHqueueschedulerMqusage) -> Self {
        self.base.params.insert(
            "mqusage".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mqusage_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mqusage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors(mut self, val: TopHqueueschedulerPdgRpcignoreerrors) -> Self {
        self.base.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_priority(mut self, val: TopHqueueschedulerHqueuePriority) -> Self {
        self.base.params.insert(
            "hqueue_priority".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_priority_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_priority".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_assign_to(mut self, val: TopHqueueschedulerHqueueAssignTo) -> Self {
        self.base.params.insert(
            "hqueue_assign_to".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_assign_to_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_assign_to".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_createcontainerjob(
        mut self,
        val: TopHqueueschedulerHqueueCreatecontainerjob,
    ) -> Self {
        self.base.params.insert(
            "hqueue_createcontainerjob".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_createcontainerjob_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_createcontainerjob".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_echandleby(mut self, val: TopHqueueschedulerHqueueEchandleby) -> Self {
        self.base.params.insert(
            "hqueue_echandleby".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_echandleby_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_echandleby".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_address(mut self, val: &str) -> Self {
        self.base.params.insert(
            "address".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_address_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "address".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jobname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "jobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_jobname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "jobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_description(mut self, val: &str) -> Self {
        self.base.params.insert(
            "description".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_description_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "description".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_workingdir(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_workingdir".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_workingdir_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_workingdir".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_mapzone(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_mapzone".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_mapzone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_mapzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_transferroot(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pdg_transferroot".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pdg_transferroot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_transferroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localsharedroot_win(mut self, val: &str) -> Self {
        self.base.params.insert(
            "localsharedroot_win".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localsharedroot_win_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localsharedroot_win".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localsharedroot_macosx(mut self, val: &str) -> Self {
        self.base.params.insert(
            "localsharedroot_macosx".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localsharedroot_macosx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localsharedroot_macosx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_localsharedroot_linux(mut self, val: &str) -> Self {
        self.base.params.insert(
            "localsharedroot_linux".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_localsharedroot_linux_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "localsharedroot_linux".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hfspathuniversal(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hfspathuniversal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hfspathuniversal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hfspathuniversal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hfs_linux_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hfs_linux_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hfs_linux_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hfs_linux_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hfs_macosx_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hfs_macosx_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hfs_macosx_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hfs_macosx_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hfs_windows_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hfs_windows_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hfs_windows_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hfs_windows_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythonbincustomuniversal(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pythonbincustomuniversal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hythonbincustomuniversal(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hythonbincustomuniversal_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hythonbincustomuniversal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "submitjobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitjobname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobnode(mut self, val: &str) -> Self {
        self.base.params.insert(
            "submitjobnode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitjobnode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobnode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobclients(mut self, val: &str) -> Self {
        self.base.params.insert(
            "submitjobclients".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitjobclients_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobclients".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobgroups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "submitjobgroups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_submitjobgroups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobgroups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remotegraphname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "remotegraphname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_remotegraphname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remotegraphname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mqaddr(mut self, val: &str) -> Self {
        self.base.params.insert(
            "mqaddr".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_mqaddr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mqaddr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_clients(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_clients".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_clients_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_clients".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_client_groups(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_client_groups".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_client_groups_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_client_groups".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_containerjobname(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_containerjobname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_containerjobname_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_containerjobname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_description(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_description".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_description_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_description".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_tags(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_tags".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_tags_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_tags".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_host(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_host".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_host_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_host".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_resname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("hqueue_resname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_resname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("hqueue_resname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_envunset(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_envunset".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_envunset_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_envunset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_env_file(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_env_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_env_file_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_env_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_envname_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("hqueue_envname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_envname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("hqueue_envname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_envvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("hqueue_envvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_envvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("hqueue_envvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_conditions(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_conditions".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_conditions_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_conditions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_preshell(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_preshell".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_preshell_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_preshell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_postshell(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_postshell".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_postshell_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_postshell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_prepy(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_prepy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_prepy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_prepy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_postpy(mut self, val: &str) -> Self {
        self.base.params.insert(
            "hqueue_postpy".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_hqueue_postpy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_postpy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_pdg_usemaxtasks(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_usemaxtasks".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemaxtasks_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_usemaxtasks".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_waitforfailures(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_waitforfailures".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_waitforfailures_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_waitforfailures".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.base.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "verbose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_compressworkitemdata".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_compressworkitemdata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_validateoutputs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_validateoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_validateoutputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_validateoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_usemapzone(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_usemapzone".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemapzone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_usemapzone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uselocalsharedroot(mut self, val: bool) -> Self {
        self.base.params.insert(
            "uselocalsharedroot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocalsharedroot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uselocalsharedroot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useuniversalhfs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "useuniversalhfs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useuniversalhfs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "useuniversalhfs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesubmitjobnode(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usesubmitjobnode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesubmitjobnode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usesubmitjobnode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobfile(mut self, val: bool) -> Self {
        self.base.params.insert(
            "submitjobfile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_submitjobfile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_submitjobsetcpus(mut self, val: bool) -> Self {
        self.base.params.insert(
            "submitjobsetcpus".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_submitjobsetcpus_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "submitjobsetcpus".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledatalayerserver(mut self, val: bool) -> Self {
        self.base.params.insert(
            "enabledatalayerserver".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledatalayerserver_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "enabledatalayerserver".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createremotegraph(mut self, val: bool) -> Self {
        self.base.params.insert(
            "createremotegraph".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createremotegraph_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "createremotegraph".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetaskcallbackport(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usetaskcallbackport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetaskcallbackport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usetaskcallbackport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemqrelayport(mut self, val: bool) -> Self {
        self.base.params.insert(
            "usemqrelayport".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemqrelayport_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "usemqrelayport".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tractor_debug(mut self, val: bool) -> Self {
        self.base.params.insert(
            "tractor_debug".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tractor_debug_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tractor_debug".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_rpcrelease(mut self, val: bool) -> Self {
        self.base.params.insert(
            "pdg_rpcrelease".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_rpcrelease_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pdg_rpcrelease".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_is_cpu_number_set(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hqueue_is_CPU_number_set".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_is_cpu_number_set_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_is_CPU_number_set".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_usehoudinimaxthreads(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hqueue_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_usehoudinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_usehoudinimaxthreads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_usemaxtime(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hqueue_usemaxtime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_usemaxtime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_usemaxtime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_echandleall(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hqueue_echandleall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_echandleall_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_echandleall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_hqueue_inheritlocalenv(mut self, val: bool) -> Self {
        self.base.params.insert(
            "hqueue_inheritlocalenv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_inheritlocalenv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "hqueue_inheritlocalenv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopHqueuescheduler {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "hqueuescheduler"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
