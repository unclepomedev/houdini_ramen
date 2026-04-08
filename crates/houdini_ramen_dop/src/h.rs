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
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopHardconrel {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn set_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(0, (out.node_id, out.pin));
        self
    }

    pub fn with_restlength(mut self, val: f32) -> Self {
        self.params.insert(
            "restlength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_erp(mut self, val: f32) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rad(mut self, val: f32) -> Self {
        self.params.insert(
            "rad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "axis1".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_axis1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "axis2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_axis2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetw(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "targetw".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_targetw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetw".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxangularimpulse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "maxangularimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_maxangularimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxangularimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numangularmotors(mut self, val: i32) -> Self {
        self.params.insert(
            "numangularmotors".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numangularmotors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numangularmotors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_restlength(mut self, val: DopHardconrelParmopRestlength) -> Self {
        self.params.insert(
            "parmop_restlength".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_restlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_cfm(mut self, val: DopHardconrelParmopCfm) -> Self {
        self.params.insert(
            "parmop_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_erp(mut self, val: DopHardconrelParmopErp) -> Self {
        self.params.insert(
            "parmop_erp".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_erp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_numangularmotors(
        mut self,
        val: DopHardconrelParmopNumangularmotors,
    ) -> Self {
        self.params.insert(
            "parmop_numangularmotors".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_numangularmotors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_numangularmotors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_axis1(mut self, val: DopHardconrelParmopAxis1) -> Self {
        self.params.insert(
            "parmop_axis1".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_axis1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_axis1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_axis2(mut self, val: DopHardconrelParmopAxis2) -> Self {
        self.params.insert(
            "parmop_axis2".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_axis2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_axis2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_targetw(mut self, val: DopHardconrelParmopTargetw) -> Self {
        self.params.insert(
            "parmop_targetw".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_targetw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_targetw".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_maxangularimpulse(
        mut self,
        val: DopHardconrelParmopMaxangularimpulse,
    ) -> Self {
        self.params.insert(
            "parmop_maxangularimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maxangularimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maxangularimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_numiterations(mut self, val: DopHardconrelParmopNumiterations) -> Self {
        self.params.insert(
            "parmop_numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_disablecollisions(
        mut self,
        val: DopHardconrelParmopDisablecollisions,
    ) -> Self {
        self.params.insert(
            "parmop_disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopHardconrelDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopHardconrelSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
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
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopHardconrel {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "hardconrel"
    }
    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)> {
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

pub trait DopHardconrelOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopHardconrelOutputs for DopHardconrel {}
impl DopHardconrelOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopHardconrel> {}
