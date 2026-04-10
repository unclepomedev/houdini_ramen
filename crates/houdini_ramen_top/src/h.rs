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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopHdaprocessor {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input1".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn trigger_updatehdaparms(mut self) -> Self {
        self.params.insert(
            "updateHDAParms".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_filterparms(mut self) -> Self {
        self.params.insert(
            "filterparms".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_manageservices(mut self) -> Self {
        self.params.insert(
            "manageservices".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_serviceport(mut self, val: i32) -> Self {
        self.params.insert(
            "serviceport".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_serviceport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "serviceport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_connectiontimeout(mut self, val: i32) -> Self {
        self.params.insert(
            "connectiontimeout".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_connectiontimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectiontimeout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_itempriority(mut self, val: i32) -> Self {
        self.params.insert(
            "itempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_itempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "itempriority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_batchcount(mut self, val: i32) -> Self {
        self.params.insert(
            "batchcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_batchcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
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
    pub fn with_operatorselection(mut self, val: TopHdaprocessorOperatorselection) -> Self {
        self.params.insert(
            "operatorselection".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_operatorselection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operatorselection".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addhdaparms(mut self, val: TopHdaprocessorAddhdaparms) -> Self {
        self.params.insert(
            "addhdaparms".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_addhdaparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addhdaparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_pdg_cooktype(mut self, val: TopHdaprocessorPdgCooktype) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_batchmode(mut self, val: TopHdaprocessorBatchmode) -> Self {
        self.params.insert(
            "batchmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_batchmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cookwhen(mut self, val: TopHdaprocessorCookwhen) -> Self {
        self.params.insert(
            "cookwhen".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cookwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cookwhen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputsource(mut self, val: TopHdaprocessorInputsource) -> Self {
        self.params.insert(
            "inputsource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inputsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputsource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_missinginput(mut self, val: TopHdaprocessorMissinginput) -> Self {
        self.params.insert(
            "missinginput".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_missinginput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "missinginput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hdatype(mut self, val: i32) -> Self {
        self.params.insert(
            "hdatype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_hdatype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hdatype".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopHdaprocessorPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopHdaprocessorPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_commandtype(mut self, val: TopHdaprocessorPdgCommandtype) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_commandtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_commandtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayedparms(mut self, val: &str) -> Self {
        self.params.insert(
            "displayedparms".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displayedparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayedparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expressionparms(mut self, val: &str) -> Self {
        self.params.insert(
            "expressionparms".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_expressionparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expressionparms".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_serviceaddress(mut self, val: &str) -> Self {
        self.params.insert(
            "serviceaddress".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_serviceaddress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "serviceaddress".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_templatenode(mut self, val: &str) -> Self {
        self.params.insert(
            "templatenode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_templatenode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "templatenode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputfile(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_operatortype(mut self, val: &str) -> Self {
        self.params.insert(
            "operatortype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_operatortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "operatortype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgservicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdgservicename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdgservicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdgservicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputfiletag(mut self, val: &str) -> Self {
        self.params.insert(
            "inputfiletag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inputfiletag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inputfiletag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("customfile{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("customfile{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hdasopname(mut self, val: &str) -> Self {
        self.params.insert(
            "hdasopname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hdasopname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hdasopname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("outputfile{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputfile{}", index1),
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
    pub fn with_outputfile(mut self, val: &str) -> Self {
        self.params.insert(
            "outputfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtag(mut self, val: &str) -> Self {
        self.params.insert(
            "outputtag".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputtag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_perffile(mut self, val: &str) -> Self {
        self.params.insert(
            "perffile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_perffile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "perffile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_debughipfile(mut self, val: &str) -> Self {
        self.params.insert(
            "debughipfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_debughipfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "debughipfile".to_string(),
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
    pub fn with_usehdaservice(mut self, val: bool) -> Self {
        self.params.insert(
            "usehdaservice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usehdaservice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usehdaservice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autoexpression(mut self, val: bool) -> Self {
        self.params.insert(
            "autoexpression".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoexpression_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoexpression".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_generatefromitems(mut self, val: bool) -> Self {
        self.params.insert(
            "generatefromitems".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_generatefromitems_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "generatefromitems".to_string(),
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
    pub fn with_enablepdgservice(mut self, val: bool) -> Self {
        self.params.insert(
            "enablepdgservice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablepdgservice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablepdgservice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inprocesscleanup(mut self, val: bool) -> Self {
        self.params.insert(
            "inprocesscleanup".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inprocesscleanup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inprocesscleanup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_createfileinput(mut self, val: bool) -> Self {
        self.params.insert(
            "createfileinput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfileinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfileinput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usecustomfile_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("usecustomfile{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustomfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("usecustomfile{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_writeoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "writeoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writeoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writeoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputenabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("outputenabled{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputenabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("outputenabled{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_writegeo(mut self, val: bool) -> Self {
        self.params.insert(
            "writegeo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_writegeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "writegeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableperfmon(mut self, val: bool) -> Self {
        self.params.insert(
            "enableperfmon".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableperfmon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableperfmon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_saveperffile(mut self, val: bool) -> Self {
        self.params.insert(
            "saveperffile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_saveperffile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "saveperffile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dumpdebug(mut self, val: bool) -> Self {
        self.params.insert(
            "dumpdebug".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dumpdebug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dumpdebug".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_debugoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "debugoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_debugoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "debugoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_reportnestederrors(mut self, val: bool) -> Self {
        self.params.insert(
            "reportnestederrors".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reportnestederrors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reportnestederrors".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for TopHdaprocessor {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hdaprocessor"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait TopHdaprocessorOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopHdaprocessorOutputs for TopHdaprocessor {}
impl TopHdaprocessorOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopHdaprocessor> {}

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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopHoudiniserver {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn set_input_at<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        index: usize,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(index),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn set_input_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input1".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(0),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input1<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(0));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn trigger_manageservices(mut self) -> Self {
        self.params.insert(
            "manageservices".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_commandport(mut self, val: i32) -> Self {
        self.params.insert(
            "commandport".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_commandport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "commandport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_timeout(mut self, val: i32) -> Self {
        self.params.insert(
            "timeout".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeout".to_string(),
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
    pub fn with_copyinputs(mut self, val: TopHoudiniserverCopyinputs) -> Self {
        self.params.insert(
            "copyinputs".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
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
    pub fn with_pdg_cooktype(mut self, val: TopHoudiniserverPdgCooktype) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_cooktype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicereset(mut self, val: TopHoudiniserverPdgServicereset) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_servicereset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicereset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen(mut self, val: TopHoudiniserverPdgServiceresetwhen) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_serviceresetwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_serviceresetwhen".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopHoudiniserverPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopHoudiniserverPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_feedbackpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_feedbackpattern".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackpattern".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalhost(mut self, val: &str) -> Self {
        self.params.insert(
            "externalhost".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalhost_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalhost".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_serverbinary(mut self, val: &str) -> Self {
        self.params.insert(
            "serverbinary".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_serverbinary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "serverbinary".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_servicename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "iterattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_iterattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sizeattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "sizeattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sizeattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sizeattribute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "numattribute".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numattribute".to_string(),
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
    pub fn with_iterationsfromupstream(mut self, val: bool) -> Self {
        self.params.insert(
            "iterationsfromupstream".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_iterationsfromupstream_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterationsfromupstream".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loopsequential(mut self, val: bool) -> Self {
        self.params.insert(
            "loopsequential".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_loopsequential_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loopsequential".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_feedbackattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_feedbackfiles(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_feedbackfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_feedbackfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_feedbackfiles".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_externalserver(mut self, val: bool) -> Self {
        self.params.insert(
            "externalserver".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_externalserver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalserver".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for TopHoudiniserver {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "houdiniserver"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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

pub trait TopHoudiniserverOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "output"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl TopHoudiniserverOutputs for TopHoudiniserver {}
impl TopHoudiniserverOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<TopHoudiniserver> {}

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
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl TopHqueuescheduler {
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

    pub fn add_spare<S: Into<houdini_ramen_core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.spare_params.push(spare.into());
        self
    }

    pub fn trigger_loadpathmap(mut self) -> Self {
        self.params.insert(
            "loadpathmap".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_queryhq(mut self) -> Self {
        self.params.insert(
            "queryhq".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_submitjob(mut self) -> Self {
        self.params.insert(
            "submitjob".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_submitjobselectclients(mut self) -> Self {
        self.params.insert(
            "submitjobselectclients".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_submitjobselectgroups(mut self) -> Self {
        self.params.insert(
            "submitjobselectgroups".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_clients(mut self) -> Self {
        self.params.insert(
            "select_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_select_client_groups(mut self) -> Self {
        self.params.insert(
            "select_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_pdg_tickperiod(mut self, val: f32) -> Self {
        self.params.insert(
            "pdg_tickperiod".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdg_tickperiod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_tickperiod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_rpcbatch(mut self, val: f32) -> Self {
        self.params.insert(
            "pdg_rpcbatch".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pdg_rpcbatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcbatch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_maxtasks(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_maxtasks".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_maxtasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_maxtasks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_maxitems(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_maxitems".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_maxitems_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_maxitems".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobcpus(mut self, val: i32) -> Self {
        self.params.insert(
            "submitjobcpus".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_submitjobcpus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobcpus".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_datalayerserverport(mut self, val: i32) -> Self {
        self.params.insert(
            "datalayerserverport".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_datalayerserverport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "datalayerserverport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_taskcallbackport(mut self, val: i32) -> Self {
        self.params.insert(
            "taskcallbackport".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskcallbackport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskcallbackport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mqrelayport(mut self, val: i32) -> Self {
        self.params.insert(
            "mqrelayport".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_mqrelayport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqrelayport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcmaxerrors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcmaxerrors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_rpctimeout(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpctimeout".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpctimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpctimeout".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_rpcretries(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcretries".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcretries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcretries".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_rpcbackoff(mut self, val: i32) -> Self {
        self.params.insert(
            "pdg_rpcbackoff".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pdg_rpcbackoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcbackoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_cpus_to_use(mut self, val: i32) -> Self {
        self.params.insert(
            "hqueue_CPUs_to_use".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_cpus_to_use_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_CPUs_to_use".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_houdinimaxthreads(mut self, val: i32) -> Self {
        self.params.insert(
            "hqueue_houdinimaxthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_houdinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_houdinimaxthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_maxtime(mut self, val: i32) -> Self {
        self.params.insert(
            "hqueue_maxtime".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_maxtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_maxtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_resqty_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("hqueue_resqty{}", index1),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_resqty_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("hqueue_resqty{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_eccustomcode(mut self, val: i32) -> Self {
        self.params.insert(
            "hqueue_eccustomcode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_eccustomcode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_eccustomcode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_triesleft(mut self, val: i32) -> Self {
        self.params.insert(
            "hqueue_triesleft".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_hqueue_triesleft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_triesleft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workitemdatasource(
        mut self,
        val: TopHqueueschedulerPdgWorkitemdatasource,
    ) -> Self {
        self.params.insert(
            "pdg_workitemdatasource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_workitemdatasource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workitemdatasource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_deletetempdir(mut self, val: TopHqueueschedulerPdgDeletetempdir) -> Self {
        self.params.insert(
            "pdg_deletetempdir".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_deletetempdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_deletetempdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_mapmode(mut self, val: TopHqueueschedulerPdgMapmode) -> Self {
        self.params.insert(
            "pdg_mapmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_mapmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_mapmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_transfertype(mut self, val: TopHqueueschedulerPdgTransfertype) -> Self {
        self.params.insert(
            "pdg_transfertype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_transfertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_transfertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pythonbin(mut self, val: TopHqueueschedulerPythonbin) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hythonbin(mut self, val: TopHqueueschedulerHythonbin) -> Self {
        self.params.insert(
            "hythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hythonbin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hythonbin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobverbosity(mut self, val: TopHqueueschedulerSubmitjobverbosity) -> Self {
        self.params.insert(
            "submitjobverbosity".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobverbosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobverbosity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobassign(mut self, val: TopHqueueschedulerSubmitjobassign) -> Self {
        self.params.insert(
            "submitjobassign".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobassign_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobassign".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedatalayerport(mut self, val: TopHqueueschedulerUsedatalayerport) -> Self {
        self.params.insert(
            "usedatalayerport".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_usedatalayerport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedatalayerport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobwhenfinished(
        mut self,
        val: TopHqueueschedulerSubmitjobwhenfinished,
    ) -> Self {
        self.params.insert(
            "submitjobwhenfinished".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_submitjobwhenfinished_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobwhenfinished".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mqusage(mut self, val: TopHqueueschedulerMqusage) -> Self {
        self.params.insert(
            "mqusage".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mqusage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqusage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors(mut self, val: TopHqueueschedulerPdgRpcignoreerrors) -> Self {
        self.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pdg_rpcignoreerrors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcignoreerrors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_priority(mut self, val: TopHqueueschedulerHqueuePriority) -> Self {
        self.params.insert(
            "hqueue_priority".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_priority_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_priority".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_assign_to(mut self, val: TopHqueueschedulerHqueueAssignTo) -> Self {
        self.params.insert(
            "hqueue_assign_to".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_assign_to_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_assign_to".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_createcontainerjob(
        mut self,
        val: TopHqueueschedulerHqueueCreatecontainerjob,
    ) -> Self {
        self.params.insert(
            "hqueue_createcontainerjob".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_createcontainerjob_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_createcontainerjob".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_echandleby(mut self, val: TopHqueueschedulerHqueueEchandleby) -> Self {
        self.params.insert(
            "hqueue_echandleby".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_hqueue_echandleby_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_echandleby".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_address(mut self, val: &str) -> Self {
        self.params.insert(
            "address".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_address_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "address".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_jobname(mut self, val: &str) -> Self {
        self.params.insert(
            "jobname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_jobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jobname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_description(mut self, val: &str) -> Self {
        self.params.insert(
            "description".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_description_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "description".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workingdir(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_workingdir".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_workingdir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_workingdir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_mapzone(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_mapzone".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_mapzone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_mapzone".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_transferroot(mut self, val: &str) -> Self {
        self.params.insert(
            "pdg_transferroot".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_transferroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_transferroot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localsharedroot_win(mut self, val: &str) -> Self {
        self.params.insert(
            "localsharedroot_win".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localsharedroot_win_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localsharedroot_win".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localsharedroot_macosx(mut self, val: &str) -> Self {
        self.params.insert(
            "localsharedroot_macosx".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localsharedroot_macosx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localsharedroot_macosx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localsharedroot_linux(mut self, val: &str) -> Self {
        self.params.insert(
            "localsharedroot_linux".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_localsharedroot_linux_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "localsharedroot_linux".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfspathuniversal(mut self, val: &str) -> Self {
        self.params.insert(
            "hfspathuniversal".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfspathuniversal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hfspathuniversal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfs_linux_path(mut self, val: &str) -> Self {
        self.params.insert(
            "hfs_linux_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfs_linux_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hfs_linux_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfs_macosx_path(mut self, val: &str) -> Self {
        self.params.insert(
            "hfs_macosx_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfs_macosx_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hfs_macosx_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfs_windows_path(mut self, val: &str) -> Self {
        self.params.insert(
            "hfs_windows_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hfs_windows_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hfs_windows_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pythonbincustomuniversal(mut self, val: &str) -> Self {
        self.params.insert(
            "pythonbincustomuniversal".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pythonbincustomuniversal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pythonbincustomuniversal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hythonbincustomuniversal(mut self, val: &str) -> Self {
        self.params.insert(
            "hythonbincustomuniversal".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hythonbincustomuniversal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hythonbincustomuniversal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobname(mut self, val: &str) -> Self {
        self.params.insert(
            "submitjobname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobnode(mut self, val: &str) -> Self {
        self.params.insert(
            "submitjobnode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobnode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobnode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobclients(mut self, val: &str) -> Self {
        self.params.insert(
            "submitjobclients".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobclients_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobclients".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobgroups(mut self, val: &str) -> Self {
        self.params.insert(
            "submitjobgroups".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobgroups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobgroups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_remotegraphname(mut self, val: &str) -> Self {
        self.params.insert(
            "remotegraphname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_remotegraphname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "remotegraphname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mqaddr(mut self, val: &str) -> Self {
        self.params.insert(
            "mqaddr".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mqaddr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mqaddr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_clients(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_clients".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_clients_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_clients".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_client_groups(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_client_groups_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_client_groups".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_containerjobname(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_containerjobname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_containerjobname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_containerjobname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_description(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_description".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_description_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_description".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_tags(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_tags".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_tags_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_tags".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_host(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_host".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_host_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_host".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_resname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("hqueue_resname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_resname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("hqueue_resname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_envunset(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_envunset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_envunset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_envunset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_env_file(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_env_file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_env_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_env_file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_envname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("hqueue_envname{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_envname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("hqueue_envname{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_envvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("hqueue_envvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_envvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("hqueue_envvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_conditions(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_conditions".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_conditions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_conditions".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_preshell(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_preshell".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_preshell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_preshell".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_postshell(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_postshell".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_postshell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_postshell".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_prepy(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_prepy".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_prepy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_prepy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_postpy(mut self, val: &str) -> Self {
        self.params.insert(
            "hqueue_postpy".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_postpy_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_postpy".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_usemaxtasks(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_usemaxtasks".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemaxtasks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_usemaxtasks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_waitforfailures(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_waitforfailures".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_waitforfailures_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_waitforfailures".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.params.insert(
            "verbose".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "verbose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_compressworkitemdata".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_compressworkitemdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_compressworkitemdata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_validateoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_validateoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_validateoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_validateoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_checkexpectedoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_checkexpectedoutputs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_usemapzone(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_usemapzone".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_usemapzone_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_usemapzone".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uselocalsharedroot(mut self, val: bool) -> Self {
        self.params.insert(
            "uselocalsharedroot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselocalsharedroot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselocalsharedroot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useuniversalhfs(mut self, val: bool) -> Self {
        self.params.insert(
            "useuniversalhfs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useuniversalhfs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useuniversalhfs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usesubmitjobnode(mut self, val: bool) -> Self {
        self.params.insert(
            "usesubmitjobnode".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesubmitjobnode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesubmitjobnode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobfile(mut self, val: bool) -> Self {
        self.params.insert(
            "submitjobfile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_submitjobfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_submitjobsetcpus(mut self, val: bool) -> Self {
        self.params.insert(
            "submitjobsetcpus".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_submitjobsetcpus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "submitjobsetcpus".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enabledatalayerserver(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledatalayerserver".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledatalayerserver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledatalayerserver".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_createremotegraph(mut self, val: bool) -> Self {
        self.params.insert(
            "createremotegraph".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createremotegraph_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createremotegraph".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetaskcallbackport(mut self, val: bool) -> Self {
        self.params.insert(
            "usetaskcallbackport".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetaskcallbackport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetaskcallbackport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usemqrelayport(mut self, val: bool) -> Self {
        self.params.insert(
            "usemqrelayport".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemqrelayport_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemqrelayport".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tractor_debug(mut self, val: bool) -> Self {
        self.params.insert(
            "tractor_debug".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tractor_debug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tractor_debug".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pdg_rpcrelease(mut self, val: bool) -> Self {
        self.params.insert(
            "pdg_rpcrelease".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pdg_rpcrelease_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pdg_rpcrelease".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_is_cpu_number_set(mut self, val: bool) -> Self {
        self.params.insert(
            "hqueue_is_CPU_number_set".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_is_cpu_number_set_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_is_CPU_number_set".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_usehoudinimaxthreads(mut self, val: bool) -> Self {
        self.params.insert(
            "hqueue_usehoudinimaxthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_usehoudinimaxthreads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_usehoudinimaxthreads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_usemaxtime(mut self, val: bool) -> Self {
        self.params.insert(
            "hqueue_usemaxtime".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_usemaxtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_usemaxtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_echandleall(mut self, val: bool) -> Self {
        self.params.insert(
            "hqueue_echandleall".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_echandleall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_echandleall".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hqueue_inheritlocalenv(mut self, val: bool) -> Self {
        self.params.insert(
            "hqueue_inheritlocalenv".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_hqueue_inheritlocalenv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hqueue_inheritlocalenv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for TopHqueuescheduler {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hqueuescheduler"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    > {
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
