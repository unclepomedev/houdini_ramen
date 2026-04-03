#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSendcommandEvaluation {
    /// Shared, Keep Changes
    SharedKeepChanges = 0,
    /// Shared, Discard Changes
    SharedDiscardChanges = 1,
    Standalone = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSendcommandExpectedoutputsmenu {
    None = 0,
    Attribute = 1,
    FileList = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSendcommandPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSendcommandPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopSendcommand {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSendcommand {
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
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pdg_cachemode(mut self, val: i32) -> Self {
        self.params.insert("pdg_cachemode".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_pdg_cachemode_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_cachemode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_evaluation(mut self, val: TopSendcommandEvaluation) -> Self {
        self.params.insert("evaluation".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_evaluation_expr(mut self, expr: &str) -> Self {
        self.params.insert("evaluation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_expectedoutputsmenu(mut self, val: TopSendcommandExpectedoutputsmenu) -> Self {
        self.params.insert("expectedoutputsmenu".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_expectedoutputsmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert("expectedoutputsmenu".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopSendcommandPdgWorkitemlabel) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopSendcommandPdgWorkitempriority) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_commandstring(mut self, val: &str) -> Self {
        self.params.insert("commandstring".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_commandstring_expr(mut self, expr: &str) -> Self {
        self.params.insert("commandstring".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_expectedoutputattr(mut self, val: &str) -> Self {
        self.params.insert("expectedoutputattr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_expectedoutputattr_expr(mut self, expr: &str) -> Self {
        self.params.insert("expectedoutputattr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_expectedoutputfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("expectedoutputfile{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_expectedoutputfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("expectedoutputfile{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyparms(mut self, val: bool) -> Self {
        self.params.insert("copyparms".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copyparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("copyparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSendcommand {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sendcommand"
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
pub enum TopSendemailSecurity {
    None = 0,
    Tls = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSendemailAuthentication {
    None = 0,
    Parameters = 1,
    File = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSendemailMessagetextsubtype {
    PlainText = 0,
    Html = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSendemailAttachmentsource {
    UpstreamOutputFile = 0,
    CustomFilePath = 1,
    None = 2,
}

#[derive(Debug, Clone)]
pub struct TopSendemail {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSendemail {
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
    pub fn with_smtp_port(mut self, val: i32) -> Self {
        self.params.insert("smtp_port".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_smtp_port_expr(mut self, expr: &str) -> Self {
        self.params.insert("smtp_port".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_security(mut self, val: TopSendemailSecurity) -> Self {
        self.params.insert("security".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_security_expr(mut self, expr: &str) -> Self {
        self.params.insert("security".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_authentication(mut self, val: TopSendemailAuthentication) -> Self {
        self.params.insert("authentication".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_authentication_expr(mut self, expr: &str) -> Self {
        self.params.insert("authentication".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_messagetextsubtype(mut self, val: TopSendemailMessagetextsubtype) -> Self {
        self.params.insert("messagetextsubtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_messagetextsubtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("messagetextsubtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attachmentsource(mut self, val: TopSendemailAttachmentsource) -> Self {
        self.params.insert("attachmentsource".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_attachmentsource_expr(mut self, expr: &str) -> Self {
        self.params.insert("attachmentsource".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_smtp_server(mut self, val: &str) -> Self {
        self.params.insert("smtp_server".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_smtp_server_expr(mut self, expr: &str) -> Self {
        self.params.insert("smtp_server".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_loginname(mut self, val: &str) -> Self {
        self.params.insert("loginname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_loginname_expr(mut self, expr: &str) -> Self {
        self.params.insert("loginname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_loginpassword(mut self, val: &str) -> Self {
        self.params.insert("loginpassword".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_loginpassword_expr(mut self, expr: &str) -> Self {
        self.params.insert("loginpassword".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_authfile(mut self, val: &str) -> Self {
        self.params.insert("authfile".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_authfile_expr(mut self, expr: &str) -> Self {
        self.params.insert("authfile".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_email_address(mut self, val: &str) -> Self {
        self.params.insert("email_address".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_email_address_expr(mut self, expr: &str) -> Self {
        self.params.insert("email_address".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_email_recipients(mut self, val: &str) -> Self {
        self.params.insert("email_recipients".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_email_recipients_expr(mut self, expr: &str) -> Self {
        self.params.insert("email_recipients".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_email_subject(mut self, val: &str) -> Self {
        self.params.insert("email_subject".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_email_subject_expr(mut self, expr: &str) -> Self {
        self.params.insert("email_subject".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_email_body(mut self, val: &str) -> Self {
        self.params.insert("email_body".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_email_body_expr(mut self, expr: &str) -> Self {
        self.params.insert("email_body".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attachment_tag(mut self, val: &str) -> Self {
        self.params.insert("attachment_tag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attachment_tag_expr(mut self, expr: &str) -> Self {
        self.params.insert("attachment_tag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attachmentfilepath(mut self, val: &str) -> Self {
        self.params.insert("attachmentfilepath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attachmentfilepath_expr(mut self, expr: &str) -> Self {
        self.params.insert("attachmentfilepath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_verbose(mut self, val: bool) -> Self {
        self.params.insert("verbose".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_verbose_expr(mut self, expr: &str) -> Self {
        self.params.insert("verbose".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_verifysslcerts(mut self, val: bool) -> Self {
        self.params.insert("verifysslcerts".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_verifysslcerts_expr(mut self, expr: &str) -> Self {
        self.params.insert("verifysslcerts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usessl(mut self, val: bool) -> Self {
        self.params.insert("usessl".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usessl_expr(mut self, expr: &str) -> Self {
        self.params.insert("usessl".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSendemail {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sendemail"
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
pub enum TopServicecreatePdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreateServiceexists {
    None = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreateOwner {
    Session = 0,
    Scheduler = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreateMemorylimittype {
    None = 0,
    ResetClient = 1,
    RestartClient = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreatePortmode {
    Automatic = 0,
    Manual = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreateMqserverloglevel {
    None = 0,
    Errors = 1,
    Warnings = 2,
    All = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreateClientlogtype {
    None = 0,
    StandardOutput = 1,
    File = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreatePdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicecreatePdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopServicecreate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopServicecreate {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_openpdgservicepanel(mut self) -> Self {
        self.params.insert("openpdgservicepanel".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_connectiontimeout(mut self, val: f32) -> Self {
        self.params.insert("connectiontimeout".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_connectiontimeout_expr(mut self, expr: &str) -> Self {
        self.params.insert("connectiontimeout".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_poolsize(mut self, val: i32) -> Self {
        self.params.insert("poolsize".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_poolsize_expr(mut self, expr: &str) -> Self {
        self.params.insert("poolsize".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_memorylimit(mut self, val: i32) -> Self {
        self.params.insert("memorylimit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_memorylimit_expr(mut self, expr: &str) -> Self {
        self.params.insert("memorylimit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_port(mut self, val: i32) -> Self {
        self.params.insert("port".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_port_expr(mut self, expr: &str) -> Self {
        self.params.insert("port".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pdg_cooktype(mut self, val: TopServicecreatePdgCooktype) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_serviceexists(mut self, val: TopServicecreateServiceexists) -> Self {
        self.params.insert("serviceexists".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_serviceexists_expr(mut self, expr: &str) -> Self {
        self.params.insert("serviceexists".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_owner(mut self, val: TopServicecreateOwner) -> Self {
        self.params.insert("owner".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_owner_expr(mut self, expr: &str) -> Self {
        self.params.insert("owner".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_type(mut self, val: i32) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_memorylimittype(mut self, val: TopServicecreateMemorylimittype) -> Self {
        self.params.insert("memorylimittype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_memorylimittype_expr(mut self, expr: &str) -> Self {
        self.params.insert("memorylimittype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_portmode(mut self, val: TopServicecreatePortmode) -> Self {
        self.params.insert("portmode".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_portmode_expr(mut self, expr: &str) -> Self {
        self.params.insert("portmode".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mqserverloglevel(mut self, val: TopServicecreateMqserverloglevel) -> Self {
        self.params.insert("mqserverloglevel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_mqserverloglevel_expr(mut self, expr: &str) -> Self {
        self.params.insert("mqserverloglevel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clientlogtype(mut self, val: TopServicecreateClientlogtype) -> Self {
        self.params.insert("clientlogtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_clientlogtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("clientlogtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopServicecreatePdgWorkitemlabel) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopServicecreatePdgWorkitempriority) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_scheduler(mut self, val: &str) -> Self {
        self.params.insert("scheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("scheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert("name".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert("name".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extraarguments(mut self, val: &str) -> Self {
        self.params.insert("extraarguments".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_extraarguments_expr(mut self, expr: &str) -> Self {
        self.params.insert("extraarguments".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_envname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("envname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_envname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("envname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_envvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("envvalue{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_envvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("envvalue{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_mqserverlogdirectory(mut self, val: &str) -> Self {
        self.params.insert("mqserverlogdirectory".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_mqserverlogdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert("mqserverlogdirectory".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clientlogdirectory(mut self, val: &str) -> Self {
        self.params.insert("clientlogdirectory".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clientlogdirectory_expr(mut self, expr: &str) -> Self {
        self.params.insert("clientlogdirectory".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyoutputs(mut self, val: bool) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copyoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_persistent(mut self, val: bool) -> Self {
        self.params.insert("persistent".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_persistent_expr(mut self, expr: &str) -> Self {
        self.params.insert("persistent".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_autostart(mut self, val: bool) -> Self {
        self.params.insert("autostart".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_autostart_expr(mut self, expr: &str) -> Self {
        self.params.insert("autostart".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopServicecreate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "servicecreate"
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
#[allow(clippy::wrong_self_convention)]
pub trait TopServicecreateInnerExt {
    fn servicecreate(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> TopServicecreateInnerExt for crate::core::graph::InnerGraph<'a> {
    fn servicecreate(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("servicecreate")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicedeletePdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicedeleteServicenotexists {
    None = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicedeletePdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicedeletePdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopServicedelete {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopServicedelete {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_openpdgservicepanel(mut self) -> Self {
        self.params.insert("openpdgservicepanel".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Int parameters ---
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pdg_cooktype(mut self, val: TopServicedeletePdgCooktype) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_servicenotexists(mut self, val: TopServicedeleteServicenotexists) -> Self {
        self.params.insert("servicenotexists".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_servicenotexists_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicenotexists".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopServicedeletePdgWorkitemlabel) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopServicedeletePdgWorkitempriority) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_servicename(mut self, val: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyoutputs(mut self, val: bool) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copyoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopServicedelete {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "servicedelete"
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
pub enum TopServiceresetPdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServiceresetServicestopped {
    None = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServiceresetResettype {
    Reset = 0,
    Restart = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServiceresetPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServiceresetPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopServicereset {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopServicereset {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_openpdgservicepanel(mut self) -> Self {
        self.params.insert("openpdgservicepanel".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Int parameters ---
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pdg_cooktype(mut self, val: TopServiceresetPdgCooktype) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_servicestopped(mut self, val: TopServiceresetServicestopped) -> Self {
        self.params.insert("servicestopped".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_servicestopped_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicestopped".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resettype(mut self, val: TopServiceresetResettype) -> Self {
        self.params.insert("resettype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_resettype_expr(mut self, expr: &str) -> Self {
        self.params.insert("resettype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopServiceresetPdgWorkitemlabel) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopServiceresetPdgWorkitempriority) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_servicename(mut self, val: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_clientname(mut self, val: &str) -> Self {
        self.params.insert("clientname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_clientname_expr(mut self, expr: &str) -> Self {
        self.params.insert("clientname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyoutputs(mut self, val: bool) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copyoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_useclientname(mut self, val: bool) -> Self {
        self.params.insert("useclientname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_useclientname_expr(mut self, expr: &str) -> Self {
        self.params.insert("useclientname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopServicereset {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "servicereset"
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
pub enum TopServicestartPdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicestartServicerunning {
    None = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicestartPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicestartPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopServicestart {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopServicestart {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_openpdgservicepanel(mut self) -> Self {
        self.params.insert("openpdgservicepanel".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Int parameters ---
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pdg_cooktype(mut self, val: TopServicestartPdgCooktype) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_servicerunning(mut self, val: TopServicestartServicerunning) -> Self {
        self.params.insert("servicerunning".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_servicerunning_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicerunning".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopServicestartPdgWorkitemlabel) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopServicestartPdgWorkitempriority) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_servicename(mut self, val: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyoutputs(mut self, val: bool) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copyoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopServicestart {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "servicestart"
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
pub enum TopServicestopPdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicestopServicestopped {
    None = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicestopPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopServicestopPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopServicestop {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopServicestop {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_openpdgservicepanel(mut self) -> Self {
        self.params.insert("openpdgservicepanel".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Int parameters ---
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_pdg_cooktype(mut self, val: TopServicestopPdgCooktype) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_servicestopped(mut self, val: TopServicestopServicestopped) -> Self {
        self.params.insert("servicestopped".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_servicestopped_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicestopped".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopServicestopPdgWorkitemlabel) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopServicestopPdgWorkitempriority) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_servicename(mut self, val: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert("servicename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_copyoutputs(mut self, val: bool) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_copyoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("copyoutputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopServicestop {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "servicestop"
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
pub struct TopShopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShopnet {
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


}

impl crate::core::types::HoudiniNode for TopShopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shopnet"
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
pub struct TopShotguncreate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShotguncreate {
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


    // --- Menu parameters ---
    pub fn with_pdg_workitemgeneration(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitemgeneration".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_pdg_workitemgeneration_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemgeneration".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_entitytype(mut self, val: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_entitytype_expr(mut self, expr: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_datafield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("datafield_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_datafield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("datafield_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_datavalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("datavalue_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_datavalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("datavalue_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_returnfield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("returnfield_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_returnfield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("returnfield_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopShotguncreate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotguncreate"
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
pub struct TopShotgundelete {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShotgundelete {
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
    pub fn with_entityid(mut self, val: i32) -> Self {
        self.params.insert("entityid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_entityid_expr(mut self, expr: &str) -> Self {
        self.params.insert("entityid".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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

    // --- String parameters ---
    pub fn with_entitytype(mut self, val: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_entitytype_expr(mut self, expr: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopShotgundelete {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotgundelete"
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
pub enum TopShotgundownloadLookupby {
    Attachment = 0,
    Id = 1,
}

#[derive(Debug, Clone)]
pub struct TopShotgundownload {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShotgundownload {
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
    pub fn with_attachmentid(mut self, val: i32) -> Self {
        self.params.insert("attachmentid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_attachmentid_expr(mut self, expr: &str) -> Self {
        self.params.insert("attachmentid".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_lookupby(mut self, val: TopShotgundownloadLookupby) -> Self {
        self.params.insert("lookupby".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_lookupby_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupby".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_attachment(mut self, val: &str) -> Self {
        self.params.insert("attachment".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attachment_expr(mut self, expr: &str) -> Self {
        self.params.insert("attachment".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_path(mut self, val: &str) -> Self {
        self.params.insert("path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_filename(mut self, val: &str) -> Self {
        self.params.insert("filename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filename_expr(mut self, expr: &str) -> Self {
        self.params.insert("filename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_modifyfilename(mut self, val: bool) -> Self {
        self.params.insert("modifyfilename".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_modifyfilename_expr(mut self, expr: &str) -> Self {
        self.params.insert("modifyfilename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopShotgundownload {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotgundownload"
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
pub enum TopShotgunfindQueryop {
    Is = 0,
    IsNot = 1,
    LessThan = 2,
    GreaterThan = 3,
    Contains = 4,
    NotContains = 5,
    StartsWith = 6,
    EndsWith = 7,
    Between = 8,
    NotBetween = 9,
    InLast = 10,
    InNext = 11,
    In = 12,
    TypeIs = 13,
    TypeIsNot = 14,
    InCalendarDay = 15,
    InCalendarWeek = 16,
    InCalendarMonth = 17,
    NameContains = 18,
    NameNotContains = 19,
    NameStartsWith = 20,
    NameEndsWith = 21,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopShotgunfindFilteroperator {
    All = 0,
    Any = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopShotgunfindOrderdir {
    Ascending = 0,
    Descending = 1,
}

#[derive(Debug, Clone)]
pub struct TopShotgunfind {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShotgunfind {
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
    pub fn with_limit(mut self, val: i32) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_queryop_inst(mut self, index1: usize, val: TopShotgunfindQueryop) -> Self {
        self.params.insert(format!("queryop_{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_queryop_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("queryop_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_filteroperator(mut self, val: TopShotgunfindFilteroperator) -> Self {
        self.params.insert("filteroperator".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_filteroperator_expr(mut self, expr: &str) -> Self {
        self.params.insert("filteroperator".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orderdir_inst(mut self, index1: usize, val: TopShotgunfindOrderdir) -> Self {
        self.params.insert(format!("orderdir_{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_orderdir_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("orderdir_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_entitytype(mut self, val: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_entitytype_expr(mut self, expr: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_queryfield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("queryfield_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_queryfield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("queryfield_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_queryvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("queryvalue_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_queryvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("queryvalue_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_returnfield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("returnfield_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_returnfield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("returnfield_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orderfield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("orderfield_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_orderfield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("orderfield_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_findall(mut self, val: bool) -> Self {
        self.params.insert("findall".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_findall_expr(mut self, expr: &str) -> Self {
        self.params.insert("findall".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_renameattribute(mut self, val: bool) -> Self {
        self.params.insert("renameattribute".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_renameattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("renameattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_retiredonly(mut self, val: bool) -> Self {
        self.params.insert("retiredonly".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_retiredonly_expr(mut self, expr: &str) -> Self {
        self.params.insert("retiredonly".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_includearchivedprojects(mut self, val: bool) -> Self {
        self.params.insert("includearchivedprojects".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_includearchivedprojects_expr(mut self, expr: &str) -> Self {
        self.params.insert("includearchivedprojects".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopShotgunfind {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotgunfind"
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
pub enum TopShotgunserverCopyinputs {
    NoIterations = 0,
    FirstIteration = 1,
    AllIterations = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopShotgunserverPdgCooktype {
    /// Shared Server (Deprecated)
    SharedServerDeprecated = 0,
    Service = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopShotgunserverPdgServicereset {
    None = 0,
    ResetClient = 1,
    RestartClient = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopShotgunserverPdgServiceresetwhen {
    BeforeCook = 0,
    AfterCook = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopShotgunserverPdgWorkitemlabel {
    UseDefault = 0,
    InheritFromUpstreamItem = 1,
    CustomExpression = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopShotgunserverPdgWorkitempriority {
    InheritFromUpstreamItem = 0,
    CustomExpression = 1,
}

#[derive(Debug, Clone)]
pub struct TopShotgunserver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShotgunserver {
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


    // --- Button parameters ---
    pub fn trigger_manageservices(mut self) -> Self {
        self.params.insert("manageservices".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Int parameters ---
    pub fn with_iterations(mut self, val: i32) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_iterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_commandport(mut self, val: i32) -> Self {
        self.params.insert("commandport".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_commandport_expr(mut self, expr: &str) -> Self {
        self.params.insert("commandport".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_timeout(mut self, val: i32) -> Self {
        self.params.insert("timeout".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_timeout_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeout".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_schedulewhen(mut self, val: i32) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_schedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_schedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriorityexpr(mut self, val: i32) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pdg_workitempriorityexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriorityexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_copyinputs(mut self, val: TopShotgunserverCopyinputs) -> Self {
        self.params.insert("copyinputs".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_copyinputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("copyinputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopShotgunserverPdgCooktype) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_cooktype_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_cooktype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_servicereset(mut self, val: TopShotgunserverPdgServicereset) -> Self {
        self.params.insert("pdg_servicereset".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_servicereset_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_servicereset".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_serviceresetwhen(mut self, val: TopShotgunserverPdgServiceresetwhen) -> Self {
        self.params.insert("pdg_serviceresetwhen".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_serviceresetwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_serviceresetwhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_addjobparms(mut self, val: i32) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_addjobparms_expr(mut self, expr: &str) -> Self {
        self.params.insert("addjobparms".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabel(mut self, val: TopShotgunserverPdgWorkitemlabel) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitemlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabel".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitempriority(mut self, val: TopShotgunserverPdgWorkitempriority) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_workitempriority_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitempriority".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_pdg_feedbackpattern(mut self, val: &str) -> Self {
        self.params.insert("pdg_feedbackpattern".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_feedbackpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_feedbackpattern".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_externalhost(mut self, val: &str) -> Self {
        self.params.insert("externalhost".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_externalhost_expr(mut self, expr: &str) -> Self {
        self.params.insert("externalhost".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_url(mut self, val: &str) -> Self {
        self.params.insert("url".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_url_expr(mut self, expr: &str) -> Self {
        self.params.insert("url".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_httpproxy(mut self, val: &str) -> Self {
        self.params.insert("httpproxy".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_httpproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert("httpproxy".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cacerts(mut self, val: &str) -> Self {
        self.params.insert("cacerts".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_cacerts_expr(mut self, expr: &str) -> Self {
        self.params.insert("cacerts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_servicename(mut self, val: &str) -> Self {
        self.params.insert("pdg_servicename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_servicename_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_servicename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_iterattribute(mut self, val: &str) -> Self {
        self.params.insert("iterattribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_iterattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sizeattribute(mut self, val: &str) -> Self {
        self.params.insert("sizeattribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sizeattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("sizeattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_numattribute(mut self, val: &str) -> Self {
        self.params.insert("numattribute".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_numattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert("numattribute".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("pdgnodedep{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("pdgnodedep{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert("topscheduler".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr(mut self, val: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdg_workitemlabelexpr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_workitemlabelexpr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_iterationsfromupstream(mut self, val: bool) -> Self {
        self.params.insert("iterationsfromupstream".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_iterationsfromupstream_expr(mut self, expr: &str) -> Self {
        self.params.insert("iterationsfromupstream".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_loopsequential(mut self, val: bool) -> Self {
        self.params.insert("loopsequential".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_loopsequential_expr(mut self, expr: &str) -> Self {
        self.params.insert("loopsequential".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_feedbackattribs(mut self, val: bool) -> Self {
        self.params.insert("pdg_feedbackattribs".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_feedbackattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_feedbackattribs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_feedbackfiles(mut self, val: bool) -> Self {
        self.params.insert("pdg_feedbackfiles".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_feedbackfiles_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_feedbackfiles".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_externalserver(mut self, val: bool) -> Self {
        self.params.insert("externalserver".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_externalserver_expr(mut self, expr: &str) -> Self {
        self.params.insert("externalserver".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usehttpproxy(mut self, val: bool) -> Self {
        self.params.insert("usehttpproxy".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usehttpproxy_expr(mut self, expr: &str) -> Self {
        self.params.insert("usehttpproxy".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usecacerts(mut self, val: bool) -> Self {
        self.params.insert("usecacerts".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usecacerts_expr(mut self, expr: &str) -> Self {
        self.params.insert("usecacerts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pdg_useschedulewhen(mut self, val: bool) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_pdg_useschedulewhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_useschedulewhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopShotgunserver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotgunserver"
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
pub struct TopShotgunupdate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShotgunupdate {
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
    pub fn with_entityid(mut self, val: i32) -> Self {
        self.params.insert("entityid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_entityid_expr(mut self, expr: &str) -> Self {
        self.params.insert("entityid".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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

    // --- String parameters ---
    pub fn with_entitytype(mut self, val: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_entitytype_expr(mut self, expr: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_datafield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("datafield_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_datafield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("datafield_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_datavalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("datavalue_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_datavalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("datavalue_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_meufield_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("meufield_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_meufield_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("meufield_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_meumode_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("meumode_{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_meumode_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("meumode_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_clearvalue_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("clearvalue_{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_clearvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("clearvalue_{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usemeupdatemodes(mut self, val: bool) -> Self {
        self.params.insert("usemeupdatemodes".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usemeupdatemodes_expr(mut self, expr: &str) -> Self {
        self.params.insert("usemeupdatemodes".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopShotgunupdate {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotgunupdate"
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
pub enum TopShotgunuploadUploadtype {
    Attachment = 0,
    Thumbnail = 1,
    Filmstrip = 2,
    Movie = 3,
}

#[derive(Debug, Clone)]
pub struct TopShotgunupload {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopShotgunupload {
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
    pub fn with_entityid(mut self, val: i32) -> Self {
        self.params.insert("entityid".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_entityid_expr(mut self, expr: &str) -> Self {
        self.params.insert("entityid".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_uploadtype(mut self, val: TopShotgunuploadUploadtype) -> Self {
        self.params.insert("uploadtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uploadtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uploadtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_entitytype(mut self, val: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_entitytype_expr(mut self, expr: &str) -> Self {
        self.params.insert("entitytype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_attachmentpath(mut self, val: &str) -> Self {
        self.params.insert("attachmentpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_attachmentpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("attachmentpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_thumbnailpath(mut self, val: &str) -> Self {
        self.params.insert("thumbnailpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_thumbnailpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("thumbnailpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_filmstrippath(mut self, val: &str) -> Self {
        self.params.insert("filmstrippath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_filmstrippath_expr(mut self, expr: &str) -> Self {
        self.params.insert("filmstrippath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_moviepath(mut self, val: &str) -> Self {
        self.params.insert("moviepath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_moviepath_expr(mut self, expr: &str) -> Self {
        self.params.insert("moviepath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_fieldname(mut self, val: &str) -> Self {
        self.params.insert("fieldname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_fieldname_expr(mut self, expr: &str) -> Self {
        self.params.insert("fieldname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayname(mut self, val: &str) -> Self {
        self.params.insert("displayname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_displayname_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_taglist(mut self, val: &str) -> Self {
        self.params.insert("taglist".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_taglist_expr(mut self, expr: &str) -> Self {
        self.params.insert("taglist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usedisplayname(mut self, val: bool) -> Self {
        self.params.insert("usedisplayname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usedisplayname_expr(mut self, expr: &str) -> Self {
        self.params.insert("usedisplayname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_usetaglist(mut self, val: bool) -> Self {
        self.params.insert("usetaglist".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usetaglist_expr(mut self, expr: &str) -> Self {
        self.params.insert("usetaglist".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopShotgunupload {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "shotgunupload"
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
pub enum TopSopnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSopnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSopnetPreXform {
    CleanTransform = 0,
    CleanTranslates = 1,
    CleanRotates = 2,
    CleanScales = 3,
    /// Extract Pre-transform
    ExtractPreMinusTransform = 4,
    /// Reset Pre-transform
    ResetPreMinusTransform = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSopnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct TopSopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSopnet {
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



    // --- Float parameters ---
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert("scale".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert("roll".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert("pos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert("bank".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert("t".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert("r".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert("s".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert("p".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert("pr".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert("up".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert("display".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathorient".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_shop_materialopts(mut self, val: i32) -> Self {
        self.params.insert("shop_materialopts".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_shop_materialopts_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_materialopts".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_xord(mut self, val: TopSopnetXord) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert("xOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rord(mut self, val: TopSopnetRord) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert("rOrd".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pre_xform(mut self, val: TopSopnetPreXform) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert("pre_xform".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uparmtype(mut self, val: TopSopnetUparmtype) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert("uparmtype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("pickscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_shop_materialpath(mut self, val: &str) -> Self {
        self.params.insert("shop_materialpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_shop_materialpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("shop_materialpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_path".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookatpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookupobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert("lookup".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("pathobjpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_showlopstage(mut self, val: &str) -> Self {
        self.params.insert("showlopstage".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_showlopstage_expr(mut self, expr: &str) -> Self {
        self.params.insert("showlopstage".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("use_dcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert("picking".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert("caching".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_shadeopen(mut self, val: bool) -> Self {
        self.params.insert("vport_shadeopen".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vport_shadeopen_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_shadeopen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_vport_displayassubdiv(mut self, val: bool) -> Self {
        self.params.insert("vport_displayassubdiv".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_vport_displayassubdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert("vport_displayassubdiv".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert("tdisplay".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert("keeppos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert("childcomp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert("constraints_on".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sopnet"
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
pub enum TopSortPdgPartitionwhen {
    InputItemsAreGenerated = 0,
    InputItemsAreCooked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSortSortdirection {
    Ascending = 0,
    Descending = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSortType {
    Integer = 0,
    Float = 1,
    String = 2,
}

#[derive(Debug, Clone)]
pub struct TopSort {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSort {
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

    /// Connects to input 0: "Work Items to Sort"
    pub fn set_input_work_items_to_sort<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Work Items to Sort" and specifies the output index of the target node.
    pub fn set_input_work_items_to_sort_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_index_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("index{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_index_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("index{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_pdg_partitionwhen(mut self, val: TopSortPdgPartitionwhen) -> Self {
        self.params.insert("pdg_partitionwhen".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_pdg_partitionwhen_expr(mut self, expr: &str) -> Self {
        self.params.insert("pdg_partitionwhen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sortdirection(mut self, val: TopSortSortdirection) -> Self {
        self.params.insert("sortdirection".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sortdirection_expr(mut self, expr: &str) -> Self {
        self.params.insert("sortdirection".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_type_inst(mut self, index1: usize, val: TopSortType) -> Self {
        self.params.insert(format!("type{}", index1), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("type{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("name{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("name{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_flattenindices(mut self, val: bool) -> Self {
        self.params.insert("flattenindices".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_flattenindices_expr(mut self, expr: &str) -> Self {
        self.params.insert("flattenindices".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSort {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sort"
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
pub enum TopSplitDisplayoutputs {
    MatchedWorkItems = 0,
    UnmatchedWorkItems = 1,
}

#[derive(Debug, Clone)]
pub struct TopSplit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSplit {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_splitexpression(mut self, val: i32) -> Self {
        self.params.insert("splitexpression".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_splitexpression_expr(mut self, expr: &str) -> Self {
        self.params.insert("splitexpression".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_displayoutputs(mut self, val: TopSplitDisplayoutputs) -> Self {
        self.params.insert("displayoutputs".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_displayoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayoutputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertselection(mut self, val: bool) -> Self {
        self.params.insert("invertselection".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertselection_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertselection".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSplit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "split"
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
pub enum TopSplitbycountSplittype {
    Beginning = 0,
    Middle = 1,
    End = 2,
    Random = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSplitbycountDisplayoutputs {
    MatchedWorkItems = 0,
    UnmatchedWorkItems = 1,
}

#[derive(Debug, Clone)]
pub struct TopSplitbycount {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSplitbycount {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Int parameters ---
    pub fn with_count(mut self, val: i32) -> Self {
        self.params.insert("count".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_count_expr(mut self, expr: &str) -> Self {
        self.params.insert("count".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_seed(mut self, val: i32) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert("seed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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
    pub fn with_splittype(mut self, val: TopSplitbycountSplittype) -> Self {
        self.params.insert("splittype".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_splittype_expr(mut self, expr: &str) -> Self {
        self.params.insert("splittype".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_displayoutputs(mut self, val: TopSplitbycountDisplayoutputs) -> Self {
        self.params.insert("displayoutputs".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_displayoutputs_expr(mut self, expr: &str) -> Self {
        self.params.insert("displayoutputs".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invertselection(mut self, val: bool) -> Self {
        self.params.insert("invertselection".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invertselection_expr(mut self, expr: &str) -> Self {
        self.params.insert("invertselection".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSplitbycount {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "splitbycount"
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
pub struct TopSqlinput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSqlinput {
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


    // --- Button parameters ---
    pub fn trigger_generatequerybutton(mut self) -> Self {
        self.params.insert("generatequerybutton".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Int parameters ---
    pub fn with_tops(mut self, val: i32) -> Self {
        self.params.insert("tops".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_tops_expr(mut self, expr: &str) -> Self {
        self.params.insert("tops".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_limit(mut self, val: i32) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_limit_expr(mut self, expr: &str) -> Self {
        self.params.insert("limit".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extractindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("extractindex{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_extractindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("extractindex{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sortindex_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(format!("sortindex{}", index1), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_sortindex_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("sortindex{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
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

    // --- String parameters ---
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("pdgnodedep{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("pdgnodedep{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_driver(mut self, val: &str) -> Self {
        self.params.insert("driver".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_driver_expr(mut self, expr: &str) -> Self {
        self.params.insert("driver".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_connectionstring(mut self, val: &str) -> Self {
        self.params.insert("connectionstring".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_connectionstring_expr(mut self, expr: &str) -> Self {
        self.params.insert("connectionstring".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scriptpath(mut self, val: &str) -> Self {
        self.params.insert("scriptpath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scriptpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("scriptpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tablename(mut self, val: &str) -> Self {
        self.params.insert("tablename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tablename_expr(mut self, expr: &str) -> Self {
        self.params.insert("tablename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_colname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("colname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_colname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("colname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extracttag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("extracttag{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_extracttag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("extracttag{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extracttype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("extracttype{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_extracttype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("extracttype{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sorttype_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("sorttype{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_sorttype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("sorttype{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_querystring(mut self, val: &str) -> Self {
        self.params.insert("querystring".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_querystring_expr(mut self, expr: &str) -> Self {
        self.params.insert("querystring".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_usecustomscript(mut self, val: bool) -> Self {
        self.params.insert("usecustomscript".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_usecustomscript_expr(mut self, expr: &str) -> Self {
        self.params.insert("usecustomscript".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_extractall(mut self, val: bool) -> Self {
        self.params.insert("extractall".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_extractall_expr(mut self, expr: &str) -> Self {
        self.params.insert("extractall".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_distinct(mut self, val: bool) -> Self {
        self.params.insert("distinct".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_distinct_expr(mut self, expr: &str) -> Self {
        self.params.insert("distinct".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_orderbyenabled_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(format!("orderbyenabled{}", index1), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_orderbyenabled_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("orderbyenabled{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_overridequeryenabled(mut self, val: bool) -> Self {
        self.params.insert("overridequeryenabled".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_overridequeryenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert("overridequeryenabled".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSqlinput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sqlinput"
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
pub struct TopSqloutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSqloutput {
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
    pub fn with_tops(mut self, val: i32) -> Self {
        self.params.insert("tops".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_tops_expr(mut self, expr: &str) -> Self {
        self.params.insert("tops".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
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

    // --- String parameters ---
    pub fn with_pdgnodedep_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("pdgnodedep{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_pdgnodedep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("pdgnodedep{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_driver(mut self, val: &str) -> Self {
        self.params.insert("driver".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_driver_expr(mut self, expr: &str) -> Self {
        self.params.insert("driver".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_connectionstring(mut self, val: &str) -> Self {
        self.params.insert("connectionstring".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_connectionstring_expr(mut self, expr: &str) -> Self {
        self.params.insert("connectionstring".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_resultdatatag(mut self, val: &str) -> Self {
        self.params.insert("resultdatatag".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_resultdatatag_expr(mut self, expr: &str) -> Self {
        self.params.insert("resultdatatag".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tablename(mut self, val: &str) -> Self {
        self.params.insert("tablename".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_tablename_expr(mut self, expr: &str) -> Self {
        self.params.insert("tablename".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_colname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("colname{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_colname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("colname{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_datatag_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(format!("datatag{}", index1), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_datatag_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(format!("datatag{}", index1), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_querystring(mut self, val: &str) -> Self {
        self.params.insert("querystring".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_querystring_expr(mut self, expr: &str) -> Self {
        self.params.insert("querystring".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_overridequeryenabled(mut self, val: bool) -> Self {
        self.params.insert("overridequeryenabled".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_overridequeryenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert("overridequeryenabled".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSqloutput {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "sqloutput"
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
pub struct TopSubnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopSubnet {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input 2"
    pub fn set_input_input_2<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input 2" and specifies the output index of the target node.
    pub fn set_input_input_2_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Input 3"
    pub fn set_input_input_3<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Input 3" and specifies the output index of the target node.
    pub fn set_input_input_3_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Input 4"
    pub fn set_input_input_4<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Input 4" and specifies the output index of the target node.
    pub fn set_input_input_4_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

}

impl crate::core::types::HoudiniNode for TopSubnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "subnet"
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
#[allow(clippy::wrong_self_convention)]
pub trait TopSubnetInnerExt {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> TopSubnetInnerExt for crate::core::graph::InnerGraph<'a> {
    fn output0(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("output0")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopSwitchInvalidindex {
    Ignore = 0,
    ReportWarning = 1,
    ReportError = 2,
}

#[derive(Debug, Clone)]
pub struct TopSwitch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl TopSwitch {
    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N) -> Self {
        self.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(mut self, index: usize, target: &N, output_index: usize) -> Self {
        self.inputs.insert(index, (target.get_id(), output_index));
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }


    // --- Int parameters ---
    pub fn with_input(mut self, val: i32) -> Self {
        self.params.insert("input".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_input_expr(mut self, expr: &str) -> Self {
        self.params.insert("input".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_invalidindex(mut self, val: TopSwitchInvalidindex) -> Self {
        self.params.insert("invalidindex".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_invalidindex_expr(mut self, expr: &str) -> Self {
        self.params.insert("invalidindex".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_invalidate(mut self, val: bool) -> Self {
        self.params.insert("invalidate".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_invalidate_expr(mut self, expr: &str) -> Self {
        self.params.insert("invalidate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_keepfailed(mut self, val: bool) -> Self {
        self.params.insert("keepfailed".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_keepfailed_expr(mut self, expr: &str) -> Self {
        self.params.insert("keepfailed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for TopSwitch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "switch"
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
