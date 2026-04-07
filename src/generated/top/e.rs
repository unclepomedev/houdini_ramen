#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopEnvironmenteditVartype {
    Float = 0,
    Integer = 1,
    String = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopEnvironmenteditVarconflict {
    ReplaceExistingVariable = 0,
    KeepExistingVariable = 1,
    UpdateExistingVariable = 2,
    /// Generate Warning on Type Mis-match
    GenerateWarningOnTypeMisMinusMatch = 3,
    /// Generate Error on Type Mis-match
    GenerateErrorOnTypeMisMinusMatch = 4,
}

#[derive(Debug, Clone)]
pub struct TopEnvironmentedit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopEnvironmentedit {
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
    pub fn with_floatvarvalue_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("floatvarvalue{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_floatvarvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("floatvarvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_intvarvalue_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("intvarvalue{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_intvarvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("intvarvalue{}", index1),
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
    pub fn with_vartype_inst(mut self, index1: usize, val: TopEnvironmenteditVartype) -> Self {
        self.params.insert(
            format!("vartype{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vartype_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("vartype{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varconflict_inst(
        mut self,
        index1: usize,
        val: TopEnvironmenteditVarconflict,
    ) -> Self {
        self.params.insert(
            format!("varconflict{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_varconflict_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varconflict{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_varname_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("varname{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_varname_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varname{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strvarvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("strvarvalue{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_strvarvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("strvarvalue{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_reset(mut self, val: bool) -> Self {
        self.params.insert(
            "reset".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_varenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("varenable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_varenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("varenable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopEnvironmentedit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "environmentedit"
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
pub enum TopErrorEvaluateduring {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopErrorPdgCooktype {
    Generate = 0,
    /// Cook (In-Process)
    CookInMinusProcess = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopErrorType {
    NodeWarning = 0,
    NodeError = 1,
    WorkItemError = 2,
    WorkItemWarnings = 3,
}

#[derive(Debug, Clone)]
pub struct TopError {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopError {
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
    pub fn with_enable_inst(mut self, index1: usize, val: i32) -> Self {
        self.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Int(val),
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
    pub fn with_evaluateduring(mut self, val: TopErrorEvaluateduring) -> Self {
        self.params.insert(
            "evaluateduring".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_evaluateduring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluateduring".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pdg_cooktype(mut self, val: TopErrorPdgCooktype) -> Self {
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
    pub fn with_type_inst(mut self, index1: usize, val: TopErrorType) -> Self {
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

    // --- String parameters ---
    pub fn with_message_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("message{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_message_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("message{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for TopError {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "error"
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
pub struct TopErrorhandler {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl TopErrorhandler {
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
}

impl crate::core::types::HoudiniNode for TopErrorhandler {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "errorhandler"
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
