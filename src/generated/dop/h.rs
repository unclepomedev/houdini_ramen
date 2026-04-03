#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopRestlength {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopCfm {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopErp {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopNumangularmotors {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopAxis1 {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopAxis2 {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopTargetw {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopMaxangularimpulse {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopNumiterations {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelParmopDisablecollisions {
    /// ![BUTTONS_set_initial]Set Initial
    SetInitial = 0,
    /// ![BUTTONS_set_or_create]Set Always
    SetAlways = 1,
    /// ![BUTTONS_set_block]Set Never
    SetNever = 2,
    /// ![BUTTONS_set_nothing]Use Default
    UseDefault = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopHardconrelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopHardconrel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopHardconrel {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_restlength(mut self, val: f32) -> Self {
        self.params.insert("restlength".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("restlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert("cfm".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert("cfm".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_erp(mut self, val: f32) -> Self {
        self.params.insert("erp".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert("erp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rad(mut self, val: f32) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert("rad".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_axis1(mut self, val: [f32; 3]) -> Self {
        self.params.insert("axis1".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_axis1_expr(mut self, expr: &str) -> Self {
        self.params.insert("axis1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_axis2(mut self, val: [f32; 3]) -> Self {
        self.params.insert("axis2".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_axis2_expr(mut self, expr: &str) -> Self {
        self.params.insert("axis2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_targetw(mut self, val: [f32; 3]) -> Self {
        self.params.insert("targetw".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_targetw_expr(mut self, expr: &str) -> Self {
        self.params.insert("targetw".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxangularimpulse(mut self, val: [f32; 3]) -> Self {
        self.params.insert("maxangularimpulse".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_maxangularimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxangularimpulse".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert("color".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert("color".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_numangularmotors(mut self, val: i32) -> Self {
        self.params.insert("numangularmotors".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_numangularmotors_expr(mut self, expr: &str) -> Self {
        self.params.insert("numangularmotors".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert("numiterations".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("numiterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert("activation".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_restlength(mut self, val: DopHardconrelParmopRestlength) -> Self {
        self.params.insert("parmop_restlength".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_restlength".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_cfm(mut self, val: DopHardconrelParmopCfm) -> Self {
        self.params.insert("parmop_cfm".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_cfm".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_erp(mut self, val: DopHardconrelParmopErp) -> Self {
        self.params.insert("parmop_erp".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_erp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_numangularmotors(mut self, val: DopHardconrelParmopNumangularmotors) -> Self {
        self.params.insert("parmop_numangularmotors".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_numangularmotors_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_numangularmotors".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_axis1(mut self, val: DopHardconrelParmopAxis1) -> Self {
        self.params.insert("parmop_axis1".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_axis1_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_axis1".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_axis2(mut self, val: DopHardconrelParmopAxis2) -> Self {
        self.params.insert("parmop_axis2".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_axis2_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_axis2".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_targetw(mut self, val: DopHardconrelParmopTargetw) -> Self {
        self.params.insert("parmop_targetw".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_targetw_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_targetw".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_maxangularimpulse(mut self, val: DopHardconrelParmopMaxangularimpulse) -> Self {
        self.params.insert("parmop_maxangularimpulse".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_maxangularimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_maxangularimpulse".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_numiterations(mut self, val: DopHardconrelParmopNumiterations) -> Self {
        self.params.insert("parmop_numiterations".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_numiterations".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_parmop_disablecollisions(mut self, val: DopHardconrelParmopDisablecollisions) -> Self {
        self.params.insert("parmop_disablecollisions".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_parmop_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert("parmop_disablecollisions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_defaultparmop(mut self, val: DopHardconrelDefaultparmop) -> Self {
        self.params.insert("defaultparmop".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert("defaultparmop".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sharedata(mut self, val: DopHardconrelSharedata) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert("sharedata".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Menu(val));
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert("activationrules".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert("group".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("dataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert("disablecollisions".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert("disablecollisions".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert("showobjectlink".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert("showobjectlink".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert("uniquedataname".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for DopHardconrel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "hardconrel"
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
