#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestPerformrequest {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestPdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestHttptype {
    Get = 0,
    Post = 1,
    Put = 2,
    Head = 3,
    Delete = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestUrlparameters {
    None = 0,
    FromAttributes = 1,
    FromMultiparm = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestPayloadtype {
    None = 0,
    Attribute = 1,
    File = 2,
    CustomString = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestPayloadfiletype {
    /// Upstream Output File(s)
    UpstreamOutputFileS = 0,
    CustomFilePath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestSaveto {
    None = 0,
    Attribute = 1,
    File = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestAttributesave {
    String = 0,
    PythonObject = 1,
    Dictionary = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestFiletag {
    Automatic = 0,
    Custom = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestClientcerttype {
    None = 0,
    SingleFile = 1,
    /// Cert/Key Pair
    CertKeyPair = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestOnfailure {
    ReportError = 0,
    ReportWarning = 1,
    Ignore = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUrlrequestPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopUrlrequest {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUrlrequest {
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
    pub fn with_timeout(mut self, val: f32) -> Self {
        self.params.insert(
            "timeout".to_string(),
            crate::core::types::ParamValue::Float(val),
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
    pub fn with_retrybackoff(mut self, val: f32) -> Self {
        self.params.insert(
            "retrybackoff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_retrybackoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "retrybackoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_maxretries(mut self, val: i32) -> Self {
        self.params.insert(
            "maxretries".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxretries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxretries".to_string(),
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
    pub fn with_performrequest(mut self, val: TopUrlrequestPerformrequest) -> Self {
        self.params.insert(
            "performrequest".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_performrequest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "performrequest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopUrlrequestPdgCooktype) -> Self {
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
    pub fn with_httptype(mut self, val: TopUrlrequestHttptype) -> Self {
        self.params.insert(
            "httptype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_httptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "httptype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_urlparameters(mut self, val: TopUrlrequestUrlparameters) -> Self {
        self.params.insert(
            "urlparameters".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_urlparameters_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "urlparameters".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloadtype(mut self, val: TopUrlrequestPayloadtype) -> Self {
        self.params.insert(
            "payloadtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_payloadtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloadfiletype(mut self, val: TopUrlrequestPayloadfiletype) -> Self {
        self.params.insert(
            "payloadfiletype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_payloadfiletype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadfiletype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_saveto(mut self, val: TopUrlrequestSaveto) -> Self {
        self.params.insert(
            "saveto".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_saveto_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "saveto".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributesave(mut self, val: TopUrlrequestAttributesave) -> Self {
        self.params.insert(
            "attributesave".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attributesave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributesave".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filetag(mut self, val: TopUrlrequestFiletag) -> Self {
        self.params.insert(
            "filetag".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_clientcerttype(mut self, val: TopUrlrequestClientcerttype) -> Self {
        self.params.insert(
            "clientcerttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_clientcerttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clientcerttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onfailure(mut self, val: TopUrlrequestOnfailure) -> Self {
        self.params.insert(
            "onfailure".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_onfailure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onfailure".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopUrlrequestPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopUrlrequestPdgWorkitempriority) -> Self {
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
    pub fn with_useragent(mut self, val: &str) -> Self {
        self.params.insert(
            "useragent".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_useragent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useragent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_contenttype(mut self, val: &str) -> Self {
        self.params.insert(
            "contenttype".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_contenttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contenttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_authorization(mut self, val: &str) -> Self {
        self.params.insert(
            "authorization".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_authorization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "authorization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_baseurl(mut self, val: &str) -> Self {
        self.params.insert(
            "baseurl".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_baseurl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseurl".to_string(),
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
    pub fn with_paramname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("paramname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_paramname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("paramname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_paramvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("paramvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_paramvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("paramvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headername_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("headername{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_headername_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("headername{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_headervalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("headervalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_headervalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("headervalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloadtag(mut self, val: &str) -> Self {
        self.params.insert(
            "payloadtag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_payloadtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloadmultinames(mut self, val: &str) -> Self {
        self.params.insert(
            "payloadmultinames".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_payloadmultinames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadmultinames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloadpath(mut self, val: &str) -> Self {
        self.params.insert(
            "payloadpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_payloadpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloadattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "payloadattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_payloadattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloadcustom(mut self, val: &str) -> Self {
        self.params.insert(
            "payloadcustom".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_payloadcustom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloadcustom".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributename(mut self, val: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
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
    pub fn with_customtag(mut self, val: &str) -> Self {
        self.params.insert(
            "customtag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_customcert(mut self, val: &str) -> Self {
        self.params.insert(
            "customcert".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_customcert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customcert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clientcertsingle(mut self, val: &str) -> Self {
        self.params.insert(
            "clientcertsingle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clientcertsingle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clientcertsingle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clientcertcert(mut self, val: &str) -> Self {
        self.params.insert(
            "clientcertcert".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clientcertcert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clientcertcert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_clientcertkey(mut self, val: &str) -> Self {
        self.params.insert(
            "clientcertkey".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clientcertkey_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clientcertkey".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_statuscode(mut self, val: &str) -> Self {
        self.params.insert(
            "statuscode".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_statuscode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "statuscode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_retrystatuscodes(mut self, val: &str) -> Self {
        self.params.insert(
            "retrystatuscodes".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_retrystatuscodes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "retrystatuscodes".to_string(),
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
    pub fn with_useuseragent(mut self, val: bool) -> Self {
        self.params.insert(
            "useuseragent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useuseragent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useuseragent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecontenttype(mut self, val: bool) -> Self {
        self.params.insert(
            "usecontenttype".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecontenttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecontenttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useauthorization(mut self, val: bool) -> Self {
        self.params.insert(
            "useauthorization".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useauthorization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useauthorization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetimeout(mut self, val: bool) -> Self {
        self.params.insert(
            "usetimeout".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetimeout".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_redirection(mut self, val: bool) -> Self {
        self.params.insert(
            "redirection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_redirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "redirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepayloadmulti(mut self, val: bool) -> Self {
        self.params.insert(
            "usepayloadmulti".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepayloadmulti_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepayloadmulti".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_payloaddict(mut self, val: bool) -> Self {
        self.params.insert(
            "payloaddict".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_payloaddict_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "payloaddict".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savebinary(mut self, val: bool) -> Self {
        self.params.insert(
            "savebinary".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savebinary_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savebinary".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stream(mut self, val: bool) -> Self {
        self.params.insert(
            "stream".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stream_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stream".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_certvalidate(mut self, val: bool) -> Self {
        self.params.insert(
            "certvalidate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_certvalidate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "certvalidate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecustomcert(mut self, val: bool) -> Self {
        self.params.insert(
            "usecustomcert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecustomcert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecustomcert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savestatuscode(mut self, val: bool) -> Self {
        self.params.insert(
            "savestatuscode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savestatuscode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savestatuscode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_saveheaders(mut self, val: bool) -> Self {
        self.params.insert(
            "saveheaders".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_saveheaders_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "saveheaders".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemaxretries(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaxretries".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxretries_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaxretries".to_string(),
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

impl crate::core::types::HoudiniNode for TopUrlrequest {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "urlrequest"
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

#[derive(Debug, Clone)]
pub struct TopUsdaddassetstogallery {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdaddassetstogallery {
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
    pub fn with_variantsets(mut self, val: &str) -> Self {
        self.params.insert(
            "variantsets".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantsets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantsets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopUsdaddassetstogallery {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdaddassetstogallery"
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
pub enum TopUsdanalyzeFilesource {
    CustomFilePath = 0,
    UpstreamOutputFile = 1,
    LopNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdanalyzeUsetime {
    WorkItemFrame = 0,
    CustomTime = 1,
    HoudiniTime = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdanalyzeStorage {
    SeparateAttributes = 0,
    DictionaryAttribute = 1,
}

#[derive(Debug, Clone)]
pub struct TopUsdanalyze {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdanalyze {
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
    pub fn with_filesource(mut self, val: TopUsdanalyzeFilesource) -> Self {
        self.params.insert(
            "filesource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filesource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetime(mut self, val: TopUsdanalyzeUsetime) -> Self {
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
    pub fn with_storage(mut self, val: TopUsdanalyzeStorage) -> Self {
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
    pub fn with_sourcetag(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dictionaryname(mut self, val: &str) -> Self {
        self.params.insert(
            "dictionaryname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dictionaryname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dictionaryname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_evaluateattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "evaluateattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_evaluateattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluateattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usestats(mut self, val: bool) -> Self {
        self.params.insert(
            "usestats".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usestats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usestats".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesavepaths(mut self, val: bool) -> Self {
        self.params.insert(
            "usesavepaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesavepaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesavepaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usedeps(mut self, val: bool) -> Self {
        self.params.insert(
            "usedeps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useexternals(mut self, val: bool) -> Self {
        self.params.insert(
            "useexternals".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useexternals_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useexternals".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopUsdanalyze {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdanalyze"
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
pub enum TopUsdimportSourcefilemode {
    CustomFilePath = 0,
    UpstreamOutputFile = 1,
    LopNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdimportIndextype {
    ElementIndex = 0,
    UpstreamItemIndex = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdimportUsetime {
    WorkItemFrame = 0,
    Separator = 1,
    CustomTime = 2,
    HoudiniTime = 3,
    /// _separator_
    Separator1 = 4,
    CustomFrame = 5,
    CustomRange = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdimportAttribsamples {
    Current = 0,
    CustomRange = 1,
    All = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdimportVariantorder {
    Alphabetical = 0,
    CurrentSelectionFirst = 1,
    CurrentSelectionOnly = 2,
}

#[derive(Debug, Clone)]
pub struct TopUsdimport {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdimport {
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
    pub fn with_attribcutoff(mut self, val: i32) -> Self {
        self.params.insert(
            "attribcutoff".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_attribcutoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribcutoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_framerange(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "framerange".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_framerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribsamplerange(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "attribsamplerange".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_attribsamplerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribsamplerange".to_string(),
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
    pub fn with_sourcefilemode(mut self, val: TopUsdimportSourcefilemode) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcefilemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indextype(mut self, val: TopUsdimportIndextype) -> Self {
        self.params.insert(
            "indextype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_indextype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indextype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetime(mut self, val: TopUsdimportUsetime) -> Self {
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
    pub fn with_attribsamples(mut self, val: TopUsdimportAttribsamples) -> Self {
        self.params.insert(
            "attribsamples".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_attribsamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribsamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantorder(mut self, val: TopUsdimportVariantorder) -> Self {
        self.params.insert(
            "variantorder".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_variantorder_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantorder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcefilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcefilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetag(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_populationmask(mut self, val: &str) -> Self {
        self.params.insert(
            "populationmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_populationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "populationmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_primpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_primpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pathattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nameattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rangeattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "rangeattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rangeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rangeattrib".to_string(),
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
    pub fn with_attributenames(mut self, val: &str) -> Self {
        self.params.insert(
            "attributenames".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributenames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributenames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relationshippattern(mut self, val: &str) -> Self {
        self.params.insert(
            "relationshippattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relationshippattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relationshippattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relationshipnames(mut self, val: &str) -> Self {
        self.params.insert(
            "relationshipnames".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_relationshipnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relationshipnames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantpattern(mut self, val: &str) -> Self {
        self.params.insert(
            "variantpattern".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantpattern".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantnames(mut self, val: &str) -> Self {
        self.params.insert(
            "variantnames".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variantnames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usepopulationmask(mut self, val: bool) -> Self {
        self.params.insert(
            "usepopulationmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepopulationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepopulationmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_striplayers(mut self, val: bool) -> Self {
        self.params.insert(
            "striplayers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_striplayers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "striplayers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_evaluateattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "evaluateattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_evaluateattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluateattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addoutput(mut self, val: bool) -> Self {
        self.params.insert(
            "addoutput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addoutput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_encodenames(mut self, val: bool) -> Self {
        self.params.insert(
            "encodenames".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_encodenames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "encodenames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepathattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "usepathattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepathattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepathattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usenameattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "usenameattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenameattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenameattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userangeattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "userangeattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userangeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userangeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_indexflatten(mut self, val: bool) -> Self {
        self.params.insert(
            "indexflatten".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_indexflatten_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "indexflatten".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useattributenames(mut self, val: bool) -> Self {
        self.params.insert(
            "useattributenames".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useattributenames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useattributenames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attribsampleconstants(mut self, val: bool) -> Self {
        self.params.insert(
            "attribsampleconstants".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attribsampleconstants_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attribsampleconstants".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_userelationshipnames(mut self, val: bool) -> Self {
        self.params.insert(
            "userelationshipnames".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userelationshipnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userelationshipnames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usevariantnames(mut self, val: bool) -> Self {
        self.params.insert(
            "usevariantnames".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usevariantnames_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usevariantnames".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopUsdimport {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdimport"
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
pub enum TopUsdimportfilesSourcefilemode {
    CustomFilePath = 0,
    UpstreamFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdimportfilesMode {
    FlattenedStage = 0,
    RecursiveLayer = 1,
    SingleLayer = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdimportfilesFiletypes {
    UsdFiles = 0,
    /// Non-USD Files
    NonMinusUsdFiles = 1,
    AllFiles = 2,
}

#[derive(Debug, Clone)]
pub struct TopUsdimportfiles {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdimportfiles {
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
    pub fn with_sourcefilemode(mut self, val: TopUsdimportfilesSourcefilemode) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcefilemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: TopUsdimportfilesMode) -> Self {
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
    pub fn with_filetypes(mut self, val: TopUsdimportfilesFiletypes) -> Self {
        self.params.insert(
            "filetypes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filetypes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filetypes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_sourcefilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcefilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetag(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
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
    pub fn with_overrideoutputfiletag(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideoutputfiletag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideoutputfiletag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideoutputfiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopUsdimportfiles {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdimportfiles"
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
pub enum TopUsdimportprimsSourcefilemode {
    CustomFilePath = 0,
    UpstreamFile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdimportprimsMode {
    FlattenedStage = 0,
    RecursiveLayer = 1,
    SingleLayer = 2,
}

#[derive(Debug, Clone)]
pub struct TopUsdimportprims {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdimportprims {
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
    pub fn with_sourcefilemode(mut self, val: TopUsdimportprimsSourcefilemode) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcefilemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: TopUsdimportprimsMode) -> Self {
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

    // --- String parameters ---
    pub fn with_sourcefilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcefilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetag(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_overrideoutputfiletag(mut self, val: bool) -> Self {
        self.params.insert(
            "overrideoutputfiletag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overrideoutputfiletag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overrideoutputfiletag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopUsdimportprims {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdimportprims"
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
pub enum TopUsdmodifypathsSourcetype {
    CustomFilePath = 0,
    UpstreamFileOutput = 1,
}

#[derive(Debug, Clone)]
pub struct TopUsdmodifypaths {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdmodifypaths {
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
    pub fn with_sourcetype(mut self, val: TopUsdmodifypathsSourcetype) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetype".to_string(),
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
    pub fn with_savedest(mut self, val: &str) -> Self {
        self.params.insert(
            "savedest".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savedest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savedest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_findprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("findprefix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_findprefix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("findprefix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_replaceprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("replaceprefix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_replaceprefix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("replaceprefix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_findsuffix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("findsuffix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_findsuffix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("findsuffix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_replacesuffix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("replacesuffix{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_replacesuffix_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("replacesuffix{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pythoncode_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("pythoncode{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pythoncode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("pythoncode{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_overwriteinput(mut self, val: bool) -> Self {
        self.params.insert(
            "overwriteinput".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overwriteinput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overwriteinput".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputdest(mut self, val: bool) -> Self {
        self.params.insert(
            "outputdest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outputdest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputdest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowchaining(mut self, val: bool) -> Self {
        self.params.insert(
            "allowchaining".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowchaining_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowchaining".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopUsdmodifypaths {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdmodifypaths"
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
pub enum TopUsdrenderPdgCooktype {
    /// In-Process
    InMinusProcess = 0,
    /// Out-of-Process
    OutMinusOfMinusProcess = 1,
    Service = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderPdgUseserviceblock {
    Never = 0,
    IfServiceNameMatches = 1,
    Always = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderFilesource {
    UpstreamOutputFile = 0,
    CustomFilePath = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderResolution {
    /// None (USD Settings)
    NoneUsdSettings = 0,
    PercentageOfResolution = 1,
    SpecifiedResolution = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderFramegeneration {
    SingleFrame = 0,
    FrameRange = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderExpectedoutputsmenu {
    None = 0,
    Attribute = 1,
    FileList = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderComplexity {
    Low = 0,
    Medium = 1,
    High = 2,
    Veryhigh = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderThreadsmenu {
    UseAllProcessors = 0,
    AllProcessorsExceptOne = 1,
    CustomThreadCount = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderOverridetype {
    AddPrefixToExecutable = 0,
    ReplaceExecutable = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderVerbosity {
    /// 2 (lowest)
    N2Lowest = 0,
    /// 3
    N3 = 1,
    /// 4
    N4 = 2,
    /// 5
    N5 = 3,
    /// 6
    N6 = 4,
    /// 7
    N7 = 5,
    /// 8
    N8 = 6,
    /// 9 (highest)
    N9Highest = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderVexprofiling {
    Off = 0,
    Vex = 1,
    VexAndNanChecks = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderPdgCommandtype {
    UseDefault = 0,
    CustomScript = 1,
    CustomCommand = 2,
}

#[derive(Debug, Clone)]
pub struct TopUsdrender {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdrender {
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

    // --- Float parameters ---
    pub fn with_frameoverride(mut self, val: f32) -> Self {
        self.params.insert(
            "frameoverride".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frameoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frameoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_range(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_resolutionscale(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolutionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framesperbatch(mut self, val: i32) -> Self {
        self.params.insert(
            "framesperbatch".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_framesperbatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framesperbatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threads(mut self, val: i32) -> Self {
        self.params.insert(
            "threads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_threads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpoint".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "timelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimit".to_string(),
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

    // --- Int2 parameters ---
    pub fn with_resolutionspecific(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolutionspecific".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_resolutionspecific_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionspecific".to_string(),
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
    pub fn with_pdg_cooktype(mut self, val: TopUsdrenderPdgCooktype) -> Self {
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
    pub fn with_pdg_useserviceblock(mut self, val: TopUsdrenderPdgUseserviceblock) -> Self {
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
    pub fn with_filesource(mut self, val: TopUsdrenderFilesource) -> Self {
        self.params.insert(
            "filesource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_filesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filesource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolution(mut self, val: TopUsdrenderResolution) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framegeneration(mut self, val: TopUsdrenderFramegeneration) -> Self {
        self.params.insert(
            "framegeneration".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_framegeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framegeneration".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedoutputsmenu(mut self, val: TopUsdrenderExpectedoutputsmenu) -> Self {
        self.params.insert(
            "expectedoutputsmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_expectedoutputsmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputsmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_complexity(mut self, val: TopUsdrenderComplexity) -> Self {
        self.params.insert(
            "complexity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_complexity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "complexity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threadsmenu(mut self, val: TopUsdrenderThreadsmenu) -> Self {
        self.params.insert(
            "threadsmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_threadsmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threadsmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overridetype(mut self, val: TopUsdrenderOverridetype) -> Self {
        self.params.insert(
            "overridetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_overridetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbosity(mut self, val: TopUsdrenderVerbosity) -> Self {
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
    pub fn with_vexprofiling(mut self, val: TopUsdrenderVexprofiling) -> Self {
        self.params.insert(
            "vexprofiling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vexprofiling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexprofiling".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopUsdrenderPdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopUsdrenderPdgWorkitempriority) -> Self {
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
    pub fn with_pdg_commandtype(mut self, val: TopUsdrenderPdgCommandtype) -> Self {
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
    pub fn with_usdpath(mut self, val: &str) -> Self {
        self.params.insert(
            "usdpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_usdpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usdpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderer(mut self, val: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputimage(mut self, val: &str) -> Self {
        self.params.insert(
            "outputimage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_outputimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputimage".to_string(),
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
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_settings(mut self, val: &str) -> Self {
        self.params.insert(
            "settings".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_settings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "settings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_commandprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "commandprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_commandprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "commandprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraarguments(mut self, val: &str) -> Self {
        self.params.insert(
            "extraarguments".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extraarguments_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraarguments".to_string(),
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
    pub fn with_usecamera(mut self, val: bool) -> Self {
        self.params.insert(
            "usecamera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useframeoverride(mut self, val: bool) -> Self {
        self.params.insert(
            "useframeoverride".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useframeoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useframeoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchall(mut self, val: bool) -> Self {
        self.params.insert(
            "batchall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_batchall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createdirs(mut self, val: bool) -> Self {
        self.params.insert(
            "createdirs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createdirs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createdirs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useoutputimage(mut self, val: bool) -> Self {
        self.params.insert(
            "useoutputimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useoutputimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useoutputimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useoutputtag(mut self, val: bool) -> Self {
        self.params.insert(
            "useoutputtag".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useoutputtag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useoutputtag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expectedoutputindexing(mut self, val: bool) -> Self {
        self.params.insert(
            "expectedoutputindexing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_expectedoutputindexing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expectedoutputindexing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderertoggle(mut self, val: bool) -> Self {
        self.params.insert(
            "renderertoggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderertoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderertoggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointtoggle(mut self, val: bool) -> Self {
        self.params.insert(
            "checkpointtoggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpointtoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointtoggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addhuskcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "addhuskcommand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addhuskcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addhuskcommand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimittoggle(mut self, val: bool) -> Self {
        self.params.insert(
            "timelimittoggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timelimittoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimittoggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimitimage(mut self, val: bool) -> Self {
        self.params.insert(
            "timelimitimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timelimitimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimitimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimitnosavepartial(mut self, val: bool) -> Self {
        self.params.insert(
            "timelimitnosavepartial".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timelimitnosavepartial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimitnosavepartial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfredprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfredprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfredprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfredprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_messagetimestamps(mut self, val: bool) -> Self {
        self.params.insert(
            "messagetimestamps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_messagetimestamps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "messagetimestamps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elapsedtimestamps(mut self, val: bool) -> Self {
        self.params.insert(
            "elapsedtimestamps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_elapsedtimestamps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elapsedtimestamps".to_string(),
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

impl crate::core::types::HoudiniNode for TopUsdrender {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdrender"
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
pub enum TopUsdrenderscenePdgCooktype {
    /// In-Process
    InMinusProcess = 0,
    /// Out-of-Process
    OutMinusOfMinusProcess = 1,
    Service = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneSourcefilemode {
    CustomFilePath = 0,
    UpstreamOutputFile = 1,
    LopNode = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneFramerange {
    SingleFrame = 0,
    FrameRange = 1,
    DetectFromScene = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneOutputsource {
    CustomPath = 0,
    Primitive = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneResolution {
    /// None (USD Settings)
    NoneUsdSettings = 0,
    PercentageOfResolution = 1,
    SpecifiedResolution = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneComplexity {
    Low = 0,
    Medium = 1,
    High = 2,
    Veryhigh = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneThreadsmenu {
    UseAllProcessors = 0,
    AllProcessorsExceptOne = 1,
    CustomThreadCount = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneVerbosity {
    /// 2 (lowest)
    N2Lowest = 0,
    /// 3
    N3 = 1,
    /// 4
    N4 = 2,
    /// 5
    N5 = 3,
    /// 6
    N6 = 4,
    /// 7
    N7 = 5,
    /// 8
    N8 = 6,
    /// 9 (highest)
    N9Highest = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrendersceneVexprofiling {
    Off = 0,
    Vex = 1,
    VexAndNanChecks = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderscenePdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopUsdrenderscenePdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopUsdrenderscene {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopUsdrenderscene {
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

    // --- Float parameters ---
    pub fn with_frameoverride(mut self, val: f32) -> Self {
        self.params.insert(
            "frameoverride".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frameoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frameoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_range(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_framesperbatch(mut self, val: i32) -> Self {
        self.params.insert(
            "framesperbatch".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_framesperbatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framesperbatch".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolutionscale(mut self, val: i32) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolutionscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threads(mut self, val: i32) -> Self {
        self.params.insert(
            "threads".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_threads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpoint(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpoint".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpoint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpoint".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimit(mut self, val: i32) -> Self {
        self.params.insert(
            "timelimit".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_timelimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimit".to_string(),
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

    // --- Int2 parameters ---
    pub fn with_resolutionspecific(mut self, val: [i32; 2]) -> Self {
        self.params.insert(
            "resolutionspecific".to_string(),
            crate::core::types::ParamValue::Int2(val),
        );
        self
    }
    pub fn with_resolutionspecific_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolutionspecific".to_string(),
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
    pub fn with_pdg_cooktype(mut self, val: TopUsdrenderscenePdgCooktype) -> Self {
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
    pub fn with_sourcefilemode(mut self, val: TopUsdrendersceneSourcefilemode) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcefilemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_framerange(mut self, val: TopUsdrendersceneFramerange) -> Self {
        self.params.insert(
            "framerange".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_framerange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "framerange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outputsource(mut self, val: TopUsdrendersceneOutputsource) -> Self {
        self.params.insert(
            "outputsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_outputsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolution(mut self, val: TopUsdrendersceneResolution) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_resolution_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolution".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_complexity(mut self, val: TopUsdrendersceneComplexity) -> Self {
        self.params.insert(
            "complexity".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_complexity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "complexity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threadsmenu(mut self, val: TopUsdrendersceneThreadsmenu) -> Self {
        self.params.insert(
            "threadsmenu".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_threadsmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "threadsmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_verbosity(mut self, val: TopUsdrendersceneVerbosity) -> Self {
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
    pub fn with_vexprofiling(mut self, val: TopUsdrendersceneVexprofiling) -> Self {
        self.params.insert(
            "vexprofiling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vexprofiling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vexprofiling".to_string(),
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
    pub fn with_pdg_workitemlabel(mut self, val: TopUsdrenderscenePdgWorkitemlabel) -> Self {
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
    pub fn with_pdg_workitempriority(mut self, val: TopUsdrenderscenePdgWorkitempriority) -> Self {
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
    pub fn with_sourcefilepath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcefilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcefilepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcetag(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcetag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcetag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_loppath(mut self, val: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tempfile(mut self, val: &str) -> Self {
        self.params.insert(
            "tempfile".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_tempfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tempfile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_savestyle(mut self, val: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_savestyle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savestyle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_populationmask(mut self, val: &str) -> Self {
        self.params.insert(
            "populationmask".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_populationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "populationmask".to_string(),
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
    pub fn with_renderproduct(mut self, val: &str) -> Self {
        self.params.insert(
            "renderproduct".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderproduct_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderproduct".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributename(mut self, val: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderer(mut self, val: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_renderer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_camera(mut self, val: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_camera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "camera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_purpose(mut self, val: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_purpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "purpose".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_settings(mut self, val: &str) -> Self {
        self.params.insert(
            "settings".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_settings_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "settings".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_commandprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "commandprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_commandprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "commandprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extraarguments(mut self, val: &str) -> Self {
        self.params.insert(
            "extraarguments".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extraarguments_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraarguments".to_string(),
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
    pub fn with_fileperframe(mut self, val: bool) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fileperframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fileperframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trackprimexistence(mut self, val: bool) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_trackprimexistence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trackprimexistence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths(mut self, val: bool) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_errorsavingimplicitpaths_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorsavingimplicitpaths".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepopulationmask(mut self, val: bool) -> Self {
        self.params.insert(
            "usepopulationmask".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepopulationmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepopulationmask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useframeoverride(mut self, val: bool) -> Self {
        self.params.insert(
            "useframeoverride".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useframeoverride_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useframeoverride".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_batchall(mut self, val: bool) -> Self {
        self.params.insert(
            "batchall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_batchall_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "batchall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usecamera(mut self, val: bool) -> Self {
        self.params.insert(
            "usecamera".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usecamera_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usecamera".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_renderertoggle(mut self, val: bool) -> Self {
        self.params.insert(
            "renderertoggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_renderertoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "renderertoggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_checkpointtoggle(mut self, val: bool) -> Self {
        self.params.insert(
            "checkpointtoggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpointtoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointtoggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addhuskcommand(mut self, val: bool) -> Self {
        self.params.insert(
            "addhuskcommand".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addhuskcommand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addhuskcommand".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimittoggle(mut self, val: bool) -> Self {
        self.params.insert(
            "timelimittoggle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timelimittoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimittoggle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimitimage(mut self, val: bool) -> Self {
        self.params.insert(
            "timelimitimage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timelimitimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimitimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timelimitnosavepartial(mut self, val: bool) -> Self {
        self.params.insert(
            "timelimitnosavepartial".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timelimitnosavepartial_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timelimitnosavepartial".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_alfredprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfredprogress".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfredprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfredprogress".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_messagetimestamps(mut self, val: bool) -> Self {
        self.params.insert(
            "messagetimestamps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_messagetimestamps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "messagetimestamps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_elapsedtimestamps(mut self, val: bool) -> Self {
        self.params.insert(
            "elapsedtimestamps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_elapsedtimestamps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "elapsedtimestamps".to_string(),
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

impl crate::core::types::HoudiniNode for TopUsdrenderscene {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "usdrenderscene"
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
