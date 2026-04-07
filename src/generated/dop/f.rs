#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFanParmopT {
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
pub enum DopFanParmopDirection {
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
pub enum DopFanParmopConeangle {
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
pub enum DopFanParmopFalloffangle {
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
pub enum DopFanParmopFlux {
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
pub enum DopFanParmopFalloff {
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
pub enum DopFanParmopMaxdistance {
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
pub enum DopFanParmopSamplemode {
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
pub enum DopFanSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFanDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFanSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopFan {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopFan {
    pub const OUT_OUTPUT1: &'static str = "output1";

    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- Float parameters ---
    pub fn with_coneangle(mut self, val: f32) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloffangle(mut self, val: f32) -> Self {
        self.params.insert(
            "falloffangle".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloffangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloffangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_flux(mut self, val: f32) -> Self {
        self.params.insert(
            "flux".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_flux_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "flux".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_falloff(mut self, val: f32) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_direction(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_t(mut self, val: DopFanParmopT) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_direction(mut self, val: DopFanParmopDirection) -> Self {
        self.params.insert(
            "parmop_direction".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_direction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_direction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_coneangle(mut self, val: DopFanParmopConeangle) -> Self {
        self.params.insert(
            "parmop_coneangle".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_coneangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_coneangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_falloffangle(mut self, val: DopFanParmopFalloffangle) -> Self {
        self.params.insert(
            "parmop_falloffangle".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_falloffangle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_falloffangle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_flux(mut self, val: DopFanParmopFlux) -> Self {
        self.params.insert(
            "parmop_flux".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_flux_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_flux".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_falloff(mut self, val: DopFanParmopFalloff) -> Self {
        self.params.insert(
            "parmop_falloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_falloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_falloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_maxdistance(mut self, val: DopFanParmopMaxdistance) -> Self {
        self.params.insert(
            "parmop_maxdistance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_samplemode(mut self, val: DopFanParmopSamplemode) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplemode(mut self, val: DopFanSamplemode) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopFanDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopFanSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFan {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fan"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemattachconstraintNormalfilterorientation {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemattachconstraintSourceborderside {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemattachconstraintTargetborderside {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemattachconstraintRestinitialization {
    Zero = 0,
    Reference = 1,
    Absolute = 2,
}

#[derive(Debug, Clone)]
pub struct DopFemattachconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemattachconstraint {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_strength(mut self, val: f32) -> Self {
        self.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_normalfilterorientation(
        mut self,
        val: DopFemattachconstraintNormalfilterorientation,
    ) -> Self {
        self.params.insert(
            "normalfilterorientation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_normalfilterorientation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalfilterorientation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceborderside(mut self, val: DopFemattachconstraintSourceborderside) -> Self {
        self.params.insert(
            "sourceborderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourceborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetborderside(mut self, val: DopFemattachconstraintTargetborderside) -> Self {
        self.params.insert(
            "targetborderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restinitialization(
        mut self,
        val: DopFemattachconstraintRestinitialization,
    ) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restinitialization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constrainedobject(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedregistrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedregistrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedregistrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedregistrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalobject(mut self, val: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalregistrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "goalregistrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalregistrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalregistrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraineddistancethresholdattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constraineddistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraineddistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraineddistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedstrengthattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedstrengthattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedstrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedstrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraineddampingattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constraineddampingattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraineddampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraineddampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedenabledistancethresholdattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainedenabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainedenabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedenabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbynormaldirection(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbynormaldirection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbynormaldirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbynormaldirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbysourceborderside(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbysourceborderside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbysourceborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbysourceborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbytargetborderside(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbytargetborderside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbytargetborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbytargetborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attachtolocalspace(mut self, val: bool) -> Self {
        self.params.insert(
            "attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_attachtolocalspace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attachtolocalspace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablestrengthattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledampingattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerestdistanceattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemattachconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femattachconstraint"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemfuseconstraintType {
    Hard = 0,
    Soft = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemfuseconstraintMatchmethod {
    OrderedPointGroup = 0,
    IdentifierPointAttribute = 1,
}

#[derive(Debug, Clone)]
pub struct DopFemfuseconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemfuseconstraint {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_strength(mut self, val: f32) -> Self {
        self.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: DopFemfuseconstraintType) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchmethod(mut self, val: DopFemfuseconstraintMatchmethod) -> Self {
        self.params.insert(
            "matchmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constrainedobject(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fusepidattribnamea(mut self, val: &str) -> Self {
        self.params.insert(
            "fusepidattribnamea".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fusepidattribnamea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fusepidattribnamea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalobject(mut self, val: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "goalpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fusepidattribnameb(mut self, val: &str) -> Self {
        self.params.insert(
            "fusepidattribnameb".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fusepidattribnameb_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fusepidattribnameb".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributea(mut self, val: &str) -> Self {
        self.params.insert(
            "attributea".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributea_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "attributea".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemfuseconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femfuseconstraint"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemhybridconfigureobjectShellplanarmodel {
    Isotropic = 0,
    Orthotropic = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemhybridconfigureobjectShellbendmodel {
    Weak = 0,
    Strong = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemhybridconfigureobjectSolidmaterialmodel {
    /// Stable Neo-Hookean Variant (Organic Tissue, Large deformations)
    StableNeoMinusHookeanVariantOrganicTissueLargeDeformations = 0,
    /// Corotated Linear (Stiff Materials, Small Deformations)
    CorotatedLinearStiffMaterialsSmallDeformations = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemhybridconfigureobjectSdfUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone)]
pub struct DopFemhybridconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemhybridconfigureobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_sub_network_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_particlemassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "particlemassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlemassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlemassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlethickness(mut self, val: f32) -> Self {
        self.params.insert(
            "particlethickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlethickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlethickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roddampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "roddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodmassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "rodmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shelldampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "shelldampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shelldampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shelldampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellmassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "shellmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellstretchstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellstretchstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellstretchstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellstretchstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellshearstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellshearstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellshearstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellshearstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellweakbendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellweakbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellweakbendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellweakbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellstrongbendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellstrongbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellstrongbendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellstrongbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellanisou(mut self, val: f32) -> Self {
        self.params.insert(
            "shellanisou".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellanisou_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellanisou".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellanisov(mut self, val: f32) -> Self {
        self.params.insert(
            "shellanisov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellanisov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellanisov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidanisou(mut self, val: f32) -> Self {
        self.params.insert(
            "solidanisou".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidanisou_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidanisou".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidanisov(mut self, val: f32) -> Self {
        self.params.insert(
            "solidanisov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidanisov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidanisov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidanisow(mut self, val: f32) -> Self {
        self.params.insert(
            "solidanisow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidanisow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidanisow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsion(mut self, val: f32) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repulsion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradius(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepalpha(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaldrag(mut self, val: f32) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaldrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentdrag(mut self, val: f32) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentdrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sdf_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiuscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionradiuscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_enableparticle(mut self, val: i32) -> Self {
        self.params.insert(
            "enableparticle".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enableparticle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableparticle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerod(mut self, val: i32) -> Self {
        self.params.insert(
            "enablerod".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enablerod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableshell(mut self, val: i32) -> Self {
        self.params.insert(
            "enableshell".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enableshell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableshell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolid(mut self, val: i32) -> Self {
        self.params.insert(
            "enablesolid".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enablesolid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablesolid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepcount(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_sdf_div(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sdf_div".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sdf_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_shellplanarmodel(
        mut self,
        val: DopFemhybridconfigureobjectShellplanarmodel,
    ) -> Self {
        self.params.insert(
            "shellplanarmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shellplanarmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellplanarmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellbendmodel(mut self, val: DopFemhybridconfigureobjectShellbendmodel) -> Self {
        self.params.insert(
            "shellbendmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_shellbendmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellbendmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmaterialmodel(
        mut self,
        val: DopFemhybridconfigureobjectSolidmaterialmodel,
    ) -> Self {
        self.params.insert(
            "solidmaterialmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solidmaterialmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidmaterialmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_uniformvoxels(
        mut self,
        val: DopFemhybridconfigureobjectSdfUniformvoxels,
    ) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sdf_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_solidenableaniso(mut self, val: bool) -> Self {
        self.params.insert(
            "solidenableaniso".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solidenableaniso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidenableaniso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importrestgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importrestgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableembedding(mut self, val: bool) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableembedding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideindependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideindependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidecodependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidecodependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideself(mut self, val: bool) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselfcomponent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselfcomponent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselffracturepart(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselffracturepart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "sdf_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdf_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createqualityattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createenergyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createforceattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcollisionattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfractureattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionradiusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemhybridconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femhybridconfigureobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemhybridobjectStrainmodel {
    Small = 0,
    Large = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemhybridobjectSolidmaterialmodel {
    /// Stable Neo-Hookean Variant (Organic Tissue, Large deformations)
    StableNeoMinusHookeanVariantOrganicTissueLargeDeformations = 0,
    /// Corotated Linear (Stiff Materials, Small Deformations)
    CorotatedLinearStiffMaterialsSmallDeformations = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemhybridobjectSdfUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone)]
pub struct DopFemhybridobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemhybridobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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

    // --- Float parameters ---
    pub fn with_particlemassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "particlemassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlemassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlemassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_particlethickness(mut self, val: f32) -> Self {
        self.params.insert(
            "particlethickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlethickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlethickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roddampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "roddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodmassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "rodmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rodshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shelldampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "shelldampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shelldampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shelldampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellmassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "shellmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellthickness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shellshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shellshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shellshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shellshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strongbendstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strongbendstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strongbendstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soliddampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_soliddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soliddampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidmassdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidshapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidvolumestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidvolumestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidvolumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidanisou(mut self, val: f32) -> Self {
        self.params.insert(
            "solidanisou".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidanisou_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidanisou".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidanisov(mut self, val: f32) -> Self {
        self.params.insert(
            "solidanisov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidanisov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidanisov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidanisow(mut self, val: f32) -> Self {
        self.params.insert(
            "solidanisow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidanisow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidanisow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsion(mut self, val: f32) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repulsion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradius(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepalpha(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaldrag(mut self, val: f32) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaldrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentdrag(mut self, val: f32) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentdrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createframe(mut self, val: f32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_relativestiffness(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "relativestiffness".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_relativestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relativestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sdf_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiuscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionradiuscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sdf_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepcount(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_strainmodel(mut self, val: DopFemhybridobjectStrainmodel) -> Self {
        self.params.insert(
            "strainmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_strainmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strainmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidmaterialmodel(mut self, val: DopFemhybridobjectSolidmaterialmodel) -> Self {
        self.params.insert(
            "solidmaterialmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solidmaterialmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidmaterialmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_uniformvoxels(mut self, val: DopFemhybridobjectSdfUniformvoxels) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sdf_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_object_name(mut self, val: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enableparticle(mut self, val: bool) -> Self {
        self.params.insert(
            "enableparticle".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableparticle_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableparticle".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerod(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableshell(mut self, val: bool) -> Self {
        self.params.insert(
            "enableshell".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableshell_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableshell".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablesolid(mut self, val: bool) -> Self {
        self.params.insert(
            "enablesolid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesolid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablesolid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solidenableaniso(mut self, val: bool) -> Self {
        self.params.insert(
            "solidenableaniso".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solidenableaniso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidenableaniso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importrestgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importrestgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableembedding(mut self, val: bool) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableembedding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideindependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideindependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidecodependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidecodependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideself(mut self, val: bool) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselfcomponent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselfcomponent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselffracturepart(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselffracturepart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createqualityattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createenergyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createforceattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcollisionattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfractureattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionradiusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemhybridobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femhybridobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub struct DopFemregionconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemregionconstraint {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constrainedobject(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedregistrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedregistrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedregistrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedregistrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainstrengthattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainstrengthattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainstrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainstrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraindampingattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constraindampingattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraindampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraindampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalobject(mut self, val: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalregistrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "goalregistrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalregistrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalregistrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalstrengthattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "goalstrengthattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalstrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalstrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goaldampingattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "goaldampingattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goaldampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goaldampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_identifierattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "identifierattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_identifierattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "identifierattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_constrainedallowpartialoverlap(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainedallowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainedallowpartialoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedallowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainenablemultipliers(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainenablemultipliers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainenablemultipliers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainenablemultipliers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalallowpartialoverlap(mut self, val: bool) -> Self {
        self.params.insert(
            "goalallowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_goalallowpartialoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalallowpartialoverlap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalenablemultipliers(mut self, val: bool) -> Self {
        self.params.insert(
            "goalenablemultipliers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_goalenablemultipliers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalenablemultipliers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablematching(mut self, val: bool) -> Self {
        self.params.insert(
            "enablematching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablematching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablematching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemregionconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femregionconstraint"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemslideconstraintNormalfilterorientation {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemslideconstraintSourceborderside {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemslideconstraintTargetborderside {
    Interior = 0,
    Exterior = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemslideconstraintConnectionmodel {
    /// Two-Sided: Attract And Repel
    TwoMinusSidedAttractAndRepel = 0,
    /// One-Sided: Only Repel
    OneMinusSidedOnlyRepel = 1,
    /// One-Sided: Only Attract
    OneMinusSidedOnlyAttract = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemslideconstraintRestinitialization {
    Zero = 0,
    Reference = 1,
    Absolute = 2,
}

#[derive(Debug, Clone)]
pub struct DopFemslideconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemslideconstraint {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_strength(mut self, val: f32) -> Self {
        self.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_normalfilterorientation(
        mut self,
        val: DopFemslideconstraintNormalfilterorientation,
    ) -> Self {
        self.params.insert(
            "normalfilterorientation".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_normalfilterorientation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normalfilterorientation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceborderside(mut self, val: DopFemslideconstraintSourceborderside) -> Self {
        self.params.insert(
            "sourceborderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourceborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetborderside(mut self, val: DopFemslideconstraintTargetborderside) -> Self {
        self.params.insert(
            "targetborderside".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_targetborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_connectionmodel(mut self, val: DopFemslideconstraintConnectionmodel) -> Self {
        self.params.insert(
            "connectionmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_connectionmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "connectionmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restinitialization(mut self, val: DopFemslideconstraintRestinitialization) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_restinitialization_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restinitialization".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constrainedobject(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedregistrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedregistrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedregistrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedregistrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalobject(mut self, val: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_goalregistrationattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "goalregistrationattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_goalregistrationattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalregistrationattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraineddistancethresholdattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constraineddistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraineddistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraineddistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedstrengthattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedstrengthattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedstrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedstrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraineddampingattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "constraineddampingattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraineddampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraineddampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetprimitiveattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetprimitiveattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetprimitiveattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetsubindexattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetsubindexattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetsubindexattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetfaceidattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetfaceidattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetfaceidattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetbaryattribute(mut self, val: &str) -> Self {
        self.params.insert(
            "targetbaryattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetbaryattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetbaryattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedenabledistancethresholdattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "constrainedenabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constrainedenabledistancethresholdattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedenabledistancethresholdattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbynormaldirection(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbynormaldirection".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbynormaldirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbynormaldirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbysourceborderside(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbysourceborderside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbysourceborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbysourceborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_filterbytargetborderside(mut self, val: bool) -> Self {
        self.params.insert(
            "filterbytargetborderside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_filterbytargetborderside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "filterbytargetborderside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablestrengthattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestrengthattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablestrengthattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabledampingattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabledampingattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabledampingattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablerestdistanceattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablerestdistanceattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablerestdistanceattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enabletargetpositionattribute(mut self, val: bool) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enabletargetpositionattribute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enabletargetpositionattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemslideconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femslideconstraint"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemsolidconfigureobjectMaterialmodel {
    /// Stable Neo-Hookean Variant (Organic Tissue, Large deformations)
    StableNeoMinusHookeanVariantOrganicTissueLargeDeformations = 0,
    /// Corotated Linear (Stiff Materials, Small Deformations)
    CorotatedLinearStiffMaterialsSmallDeformations = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolidconfigureobjectSdfUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone)]
pub struct DopFemsolidconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemsolidconfigureobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_sub_network_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_solidstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_solidstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solidstiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_massdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_massdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "volumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisou(mut self, val: f32) -> Self {
        self.params.insert(
            "anisou".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisou_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisou".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisov(mut self, val: f32) -> Self {
        self.params.insert(
            "anisov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisow(mut self, val: f32) -> Self {
        self.params.insert(
            "anisow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsion(mut self, val: f32) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repulsion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradius(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepalpha(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaldrag(mut self, val: f32) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaldrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentdrag(mut self, val: f32) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentdrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvwscale(mut self, val: f32) -> Self {
        self.params.insert(
            "uvwscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uvwscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvwscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sdf_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiuscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionradiuscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiuscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ucolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ucolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sdf_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepcount(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_sdf_div(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sdf_div".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sdf_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_materialmodel(mut self, val: DopFemsolidconfigureobjectMaterialmodel) -> Self {
        self.params.insert(
            "materialmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_materialmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materialmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_uniformvoxels(
        mut self,
        val: DopFemsolidconfigureobjectSdfUniformvoxels,
    ) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sdf_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enableaniso(mut self, val: bool) -> Self {
        self.params.insert(
            "enableaniso".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableaniso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableaniso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importrestgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importrestgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableembedding(mut self, val: bool) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableembedding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideindependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideindependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidecodependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidecodependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideself(mut self, val: bool) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselfcomponent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselfcomponent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselffracturepart(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselffracturepart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "sdf_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdf_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createqualityattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createenergyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createforceattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcollisionattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfractureattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionradiusenable(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionradiusenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionradiusenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvwenable(mut self, val: bool) -> Self {
        self.params.insert(
            "uvwenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uvwenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvwenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemsolidconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femsolidconfigureobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemsolidobjectInitializebehavior {
    None = 0,
    Rubber = 1,
    OrganicMass = 2,
    Cork = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolidobjectMaterialmodel {
    /// Stable Neo-Hookean Variant (Organic Tissue, Large deformations)
    StableNeoMinusHookeanVariantOrganicTissueLargeDeformations = 0,
    /// Corotated Linear (Stiff Materials, Small Deformations)
    CorotatedLinearStiffMaterialsSmallDeformations = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolidobjectSdfUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolidobjectStrainmodel {
    Small = 0,
    Large = 1,
}

#[derive(Debug, Clone)]
pub struct DopFemsolidobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemsolidobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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

    // --- Float parameters ---
    pub fn with_stiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dampingratio".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_massdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_massdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "shapestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_shapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shapestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "volumestiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisou(mut self, val: f32) -> Self {
        self.params.insert(
            "anisou".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisou_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisou".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisov(mut self, val: f32) -> Self {
        self.params.insert(
            "anisov".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisov_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisov".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_anisow(mut self, val: f32) -> Self {
        self.params.insert(
            "anisow".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_anisow_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anisow".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_repulsion(mut self, val: f32) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_repulsion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "repulsion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetstrength(mut self, val: f32) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetstrength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetdamping(mut self, val: f32) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_targetdamping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetdamping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_tol(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_tol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_tol".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepalpha(mut self, val: f32) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdf_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepalpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fracturethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fracturethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_normaldrag(mut self, val: f32) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_normaldrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "normaldrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tangentdrag(mut self, val: f32) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tangentdrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tangentdrag".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvwscale(mut self, val: f32) -> Self {
        self.params.insert(
            "uvwscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_uvwscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvwscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createframe(mut self, val: f32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_relativestiffness(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "relativestiffness".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_relativestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relativestiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sdf_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityoffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_externalvelocityoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ucolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_ucolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ucolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_wcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_wcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_sdf_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_sweepcount(mut self, val: i32) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sdf_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_sweepcount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_sdf_div(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sdf_div".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sdf_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_initializebehavior(mut self, val: DopFemsolidobjectInitializebehavior) -> Self {
        self.params.insert(
            "initializebehavior".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initializebehavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initializebehavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_materialmodel(mut self, val: DopFemsolidobjectMaterialmodel) -> Self {
        self.params.insert(
            "materialmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_materialmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materialmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_uniformvoxels(mut self, val: DopFemsolidobjectSdfUniformvoxels) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sdf_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strainmodel(mut self, val: DopFemsolidobjectStrainmodel) -> Self {
        self.params.insert(
            "strainmodel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_strainmodel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strainmodel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_restgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetgeometrypath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetgeometrypath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry(mut self, val: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_embeddedgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "embeddedgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield(mut self, val: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_externalvelocityfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "externalvelocityfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_object_name(mut self, val: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enableaniso(mut self, val: bool) -> Self {
        self.params.insert(
            "enableaniso".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableaniso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableaniso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importrestgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importrestgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importrestgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_importtargetgeometry(mut self, val: bool) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_importtargetgeometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "importtargetgeometry".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableembedding(mut self, val: bool) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableembedding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableembedding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideindependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideindependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideindependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collidecodependent(mut self, val: bool) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collidecodependent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collidecodependent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideself(mut self, val: bool) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideself_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideself".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselfcomponent(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselfcomponent_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselfcomponent".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collideselffracturepart(mut self, val: bool) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collideselffracturepart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collideselffracturepart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sdf_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "sdf_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sdf_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdf_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createqualityattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createenergyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createforceattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcollisionattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfractureattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvwenable(mut self, val: bool) -> Self {
        self.params.insert(
            "uvwenable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uvwenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uvwenable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemsolidobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femsolidobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemsolverSolvemethod {
    Gsl = 0,
    Gnl = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolverSimulationtype {
    Quasistatic = 0,
    Dynamic = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolverIntegratortype {
    FirstOrder = 0,
    SecondOrder = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolverFloatprecision {
    /// 32-bit
    N32MinusBit = 0,
    /// 64-bit
    N64MinusBit = 1,
}

#[derive(Debug, Clone)]
pub struct DopFemsolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemsolver {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_unitlength(mut self, val: f32) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitmass(mut self, val: f32) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitmass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses(mut self, val: i32) -> Self {
        self.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_solvemethod(mut self, val: DopFemsolverSolvemethod) -> Self {
        self.params.insert(
            "solvemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solvemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_simulationtype(mut self, val: DopFemsolverSimulationtype) -> Self {
        self.params.insert(
            "simulationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_simulationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "simulationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_integratortype(mut self, val: DopFemsolverIntegratortype) -> Self {
        self.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_integratortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatprecision(mut self, val: DopFemsolverFloatprecision) -> Self {
        self.params.insert(
            "floatprecision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_floatprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "floatprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowchangingrest(mut self, val: bool) -> Self {
        self.params.insert(
            "allowchangingrest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowchangingrest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowchangingrest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemodification(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemodification".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemodification_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemodification".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemsolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femsolver"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemsolvercoreParmopSolvertype {
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
pub enum DopFemsolvercoreSolvertype {
    FiniteElements = 0,
    Cloth = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolvercoreParmopSolvemethod {
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
pub enum DopFemsolvercoreSolvemethod {
    Gsl = 0,
    Gnl = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolvercoreParmopSimulationtype {
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
pub enum DopFemsolvercoreSimulationtype {
    Quasistatic = 0,
    Dynamic = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolvercoreParmopIntegratortype {
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
pub enum DopFemsolvercoreIntegratortype {
    FirstOrder = 0,
    SecondOrder = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolvercoreParmopFloatprecision {
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
pub enum DopFemsolvercoreFloatprecision {
    /// 32 bit
    N32Bit = 0,
    /// 64 bit
    N64Bit = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFemsolvercoreParmopMinsubsteps {
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
pub enum DopFemsolvercoreParmopMaxglobalcollisionpasses {
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
pub enum DopFemsolvercoreParmopEnablecollisions {
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
pub enum DopFemsolvercoreParmopAllowchangingrest {
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
pub enum DopFemsolvercoreParmopEnablemodification {
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
pub enum DopFemsolvercoreParmopEnablefracturing {
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
pub enum DopFemsolvercoreParmopUnitlength {
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
pub enum DopFemsolvercoreParmopUnitmass {
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
pub enum DopFemsolvercoreParmopEnforcemaxglobaliterations {
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
pub enum DopFemsolvercoreParmopMaxglobaliterations {
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
pub enum DopFemsolvercoreParmopComponentpositionthreshold {
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
pub enum DopFemsolvercoreParmopComponentvelocitythreshold {
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
pub enum DopFemsolvercoreParmopComponentaccelerationthreshold {
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
pub enum DopFemsolvercoreDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone)]
pub struct DopFemsolvercore {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopFemsolvercore {
    pub const OUT_OUTPUT1: &'static str = "output1";

    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- Float parameters ---
    pub fn with_unitlength(mut self, val: f32) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unitmass(mut self, val: f32) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_unitmass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unitmass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_componentpositionthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "componentpositionthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_componentpositionthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "componentpositionthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_componentvelocitythreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "componentvelocitythreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_componentvelocitythreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "componentvelocitythreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_componentaccelerationthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "componentaccelerationthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_componentaccelerationthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "componentaccelerationthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_minsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "minsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses(mut self, val: i32) -> Self {
        self.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxglobalcollisionpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxglobaliterations(mut self, val: i32) -> Self {
        self.params.insert(
            "maxglobaliterations".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxglobaliterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxglobaliterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_solvertype(mut self, val: DopFemsolvercoreParmopSolvertype) -> Self {
        self.params.insert(
            "parmop_solvertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_solvertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_solvertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvertype(mut self, val: DopFemsolvercoreSolvertype) -> Self {
        self.params.insert(
            "solvertype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solvertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvertype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_solvemethod(mut self, val: DopFemsolvercoreParmopSolvemethod) -> Self {
        self.params.insert(
            "parmop_solvemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_solvemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_solvemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvemethod(mut self, val: DopFemsolvercoreSolvemethod) -> Self {
        self.params.insert(
            "solvemethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solvemethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvemethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_simulationtype(mut self, val: DopFemsolvercoreParmopSimulationtype) -> Self {
        self.params.insert(
            "parmop_simulationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_simulationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_simulationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_simulationtype(mut self, val: DopFemsolvercoreSimulationtype) -> Self {
        self.params.insert(
            "simulationtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_simulationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "simulationtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_integratortype(mut self, val: DopFemsolvercoreParmopIntegratortype) -> Self {
        self.params.insert(
            "parmop_integratortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_integratortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_integratortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_integratortype(mut self, val: DopFemsolvercoreIntegratortype) -> Self {
        self.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_integratortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "integratortype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_floatprecision(mut self, val: DopFemsolvercoreParmopFloatprecision) -> Self {
        self.params.insert(
            "parmop_floatprecision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_floatprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_floatprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_floatprecision(mut self, val: DopFemsolvercoreFloatprecision) -> Self {
        self.params.insert(
            "floatprecision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_floatprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "floatprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_minsubsteps(mut self, val: DopFemsolvercoreParmopMinsubsteps) -> Self {
        self.params.insert(
            "parmop_minsubsteps".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_minsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_minsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_maxglobalcollisionpasses(
        mut self,
        val: DopFemsolvercoreParmopMaxglobalcollisionpasses,
    ) -> Self {
        self.params.insert(
            "parmop_maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maxglobalcollisionpasses_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maxglobalcollisionpasses".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablecollisions(
        mut self,
        val: DopFemsolvercoreParmopEnablecollisions,
    ) -> Self {
        self.params.insert(
            "parmop_enablecollisions".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_allowchangingrest(
        mut self,
        val: DopFemsolvercoreParmopAllowchangingrest,
    ) -> Self {
        self.params.insert(
            "parmop_allowchangingrest".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_allowchangingrest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_allowchangingrest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablemodification(
        mut self,
        val: DopFemsolvercoreParmopEnablemodification,
    ) -> Self {
        self.params.insert(
            "parmop_enablemodification".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablemodification_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablemodification".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enablefracturing(
        mut self,
        val: DopFemsolvercoreParmopEnablefracturing,
    ) -> Self {
        self.params.insert(
            "parmop_enablefracturing".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_unitlength(mut self, val: DopFemsolvercoreParmopUnitlength) -> Self {
        self.params.insert(
            "parmop_unitlength".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_unitlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_unitlength".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_unitmass(mut self, val: DopFemsolvercoreParmopUnitmass) -> Self {
        self.params.insert(
            "parmop_unitmass".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_unitmass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_unitmass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_enforcemaxglobaliterations(
        mut self,
        val: DopFemsolvercoreParmopEnforcemaxglobaliterations,
    ) -> Self {
        self.params.insert(
            "parmop_enforcemaxglobaliterations".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_enforcemaxglobaliterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_enforcemaxglobaliterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_maxglobaliterations(
        mut self,
        val: DopFemsolvercoreParmopMaxglobaliterations,
    ) -> Self {
        self.params.insert(
            "parmop_maxglobaliterations".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maxglobaliterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maxglobaliterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_componentpositionthreshold(
        mut self,
        val: DopFemsolvercoreParmopComponentpositionthreshold,
    ) -> Self {
        self.params.insert(
            "parmop_componentpositionthreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_componentpositionthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_componentpositionthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_componentvelocitythreshold(
        mut self,
        val: DopFemsolvercoreParmopComponentvelocitythreshold,
    ) -> Self {
        self.params.insert(
            "parmop_componentvelocitythreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_componentvelocitythreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_componentvelocitythreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_componentaccelerationthreshold(
        mut self,
        val: DopFemsolvercoreParmopComponentaccelerationthreshold,
    ) -> Self {
        self.params.insert(
            "parmop_componentaccelerationthreshold".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_componentaccelerationthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_componentaccelerationthreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopFemsolvercoreDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_enablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablecollisions".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowchangingrest(mut self, val: bool) -> Self {
        self.params.insert(
            "allowchangingrest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowchangingrest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowchangingrest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablemodification(mut self, val: bool) -> Self {
        self.params.insert(
            "enablemodification".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablemodification_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablemodification".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablefracturing(mut self, val: bool) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablefracturing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablefracturing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enforcemaxglobaliterations(mut self, val: bool) -> Self {
        self.params.insert(
            "enforcemaxglobaliterations".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enforcemaxglobaliterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enforcemaxglobaliterations".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solverperobject(mut self, val: bool) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solverperobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemsolvercore {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femsolvercore"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFemtargetconstraintType {
    Hard = 0,
    Soft = 1,
}

#[derive(Debug, Clone)]
pub struct DopFemtargetconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFemtargetconstraint {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_stiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stiffness".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damping".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: DopFemtargetconstraintType) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_constrainedobject(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints(mut self, val: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constrainedpoints_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrainedpoints".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFemtargetconstraint {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "femtargetconstraint"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFeoutputattributesParmopCreatequalityattributes {
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
pub enum DopFeoutputattributesParmopCreateenergyattributes {
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
pub enum DopFeoutputattributesParmopCreateforceattributes {
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
pub enum DopFeoutputattributesParmopCreatecollisionattributes {
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
pub enum DopFeoutputattributesParmopCreatefractureattributes {
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
pub enum DopFeoutputattributesDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFeoutputattributesSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopFeoutputattributes {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFeoutputattributes {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_createqualityattributes(
        mut self,
        val: DopFeoutputattributesParmopCreatequalityattributes,
    ) -> Self {
        self.params.insert(
            "parmop_createqualityattributes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_createenergyattributes(
        mut self,
        val: DopFeoutputattributesParmopCreateenergyattributes,
    ) -> Self {
        self.params.insert(
            "parmop_createenergyattributes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_createforceattributes(
        mut self,
        val: DopFeoutputattributesParmopCreateforceattributes,
    ) -> Self {
        self.params.insert(
            "parmop_createforceattributes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_createcollisionattributes(
        mut self,
        val: DopFeoutputattributesParmopCreatecollisionattributes,
    ) -> Self {
        self.params.insert(
            "parmop_createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_createfractureattributes(
        mut self,
        val: DopFeoutputattributesParmopCreatefractureattributes,
    ) -> Self {
        self.params.insert(
            "parmop_createfractureattributes".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopFeoutputattributesDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopFeoutputattributesSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_createqualityattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createqualityattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createqualityattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createenergyattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createenergyattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createenergyattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createforceattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createforceattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createforceattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createcollisionattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createcollisionattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createcollisionattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createfractureattributes(mut self, val: bool) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_createfractureattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createfractureattributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFeoutputattributes {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "feoutputattributes"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub struct DopFetchdata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopFetchdata {
    pub const OUT_OUTPUT1: &'static str = "output1";

    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_srcobject(mut self, val: &str) -> Self {
        self.params.insert(
            "srcobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srcobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srcdataname(mut self, val: &str) -> Self {
        self.params.insert(
            "srcdataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srcdataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcdataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFetchdata {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fetchdata"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFieldforceParmopForceattribname {
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
pub enum DopFieldforceParmopForcescale {
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
pub enum DopFieldforceParmopTorqueattribname {
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
pub enum DopFieldforceParmopTorquescale {
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
pub enum DopFieldforceParmopUsemaxdistance {
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
pub enum DopFieldforceParmopMaxdistance {
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
pub enum DopFieldforceParmopTreataswind {
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
pub enum DopFieldforceParmopSamplemode {
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
pub enum DopFieldforceSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFieldforceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFieldforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopFieldforce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopFieldforce {
    pub const OUT_OUTPUT1: &'static str = "output1";

    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- Float parameters ---
    pub fn with_forcescale(mut self, val: f32) -> Self {
        self.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_torquescale(mut self, val: f32) -> Self {
        self.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_torquescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "torquescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxdistance(mut self, val: f32) -> Self {
        self.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidescalef(mut self, val: f32) -> Self {
        self.params.insert(
            "guidescalef".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidescalef_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidescalef".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidescalet(mut self, val: f32) -> Self {
        self.params.insert(
            "guidescalet".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidescalet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidescalet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guidecolorf(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolorf".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolorf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolorf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidecolort(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolort".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolort_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolort".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_forceattribname(mut self, val: DopFieldforceParmopForceattribname) -> Self {
        self.params.insert(
            "parmop_forceattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forceattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forceattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_forcescale(mut self, val: DopFieldforceParmopForcescale) -> Self {
        self.params.insert(
            "parmop_forcescale".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forcescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_torqueattribname(
        mut self,
        val: DopFieldforceParmopTorqueattribname,
    ) -> Self {
        self.params.insert(
            "parmop_torqueattribname".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_torqueattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_torqueattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_torquescale(mut self, val: DopFieldforceParmopTorquescale) -> Self {
        self.params.insert(
            "parmop_torquescale".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_torquescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_torquescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_usemaxdistance(mut self, val: DopFieldforceParmopUsemaxdistance) -> Self {
        self.params.insert(
            "parmop_usemaxdistance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usemaxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_usemaxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_maxdistance(mut self, val: DopFieldforceParmopMaxdistance) -> Self {
        self.params.insert(
            "parmop_maxdistance".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_treataswind(mut self, val: DopFieldforceParmopTreataswind) -> Self {
        self.params.insert(
            "parmop_treataswind".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_treataswind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_treataswind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_parmop_samplemode(mut self, val: DopFieldforceParmopSamplemode) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_samplemode(mut self, val: DopFieldforceSamplemode) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopFieldforceDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopFieldforceSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_forceattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "forceattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_forceattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forceattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_torqueattribname(mut self, val: &str) -> Self {
        self.params.insert(
            "torqueattribname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_torqueattribname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "torqueattribname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usemaxdistance(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaxdistance".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxdistance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaxdistance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_treataswind(mut self, val: bool) -> Self {
        self.params.insert(
            "treataswind".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_treataswind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "treataswind".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideshowf(mut self, val: bool) -> Self {
        self.params.insert(
            "guideshowf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideshowf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideshowf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideshowt(mut self, val: bool) -> Self {
        self.params.insert(
            "guideshowt".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideshowt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideshowt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFieldforce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fieldforce"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub struct DopFilamentobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFilamentobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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

    // --- Float parameters ---
    pub fn with_createframe(mut self, val: f32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strengthscale(mut self, val: f32) -> Self {
        self.params.insert(
            "strengthscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strengthscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strengthscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_thicknessscale(mut self, val: f32) -> Self {
        self.params.insert(
            "thicknessscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_thicknessscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "thicknessscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objectname(mut self, val: &str) -> Self {
        self.params.insert(
            "objectname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objectname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initial_geo(mut self, val: &str) -> Self {
        self.params.insert(
            "initial_geo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initial_geo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initial_geo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_animategeo(mut self, val: bool) -> Self {
        self.params.insert(
            "animategeo".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animategeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_display(mut self, val: bool) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFilamentobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "filamentobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub struct DopFilamentsolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFilamentsolver {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Object"
    pub fn set_input_object<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Object" and specifies the output index of the target node.
    pub fn set_input_object_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_object_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Pre-Solve"
    pub fn set_input_pre_solve<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Pre-Solve" and specifies the output index of the target node.
    pub fn set_input_pre_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_pre_solve_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: "Post-Solve"
    pub fn set_input_post_solve<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "Post-Solve" and specifies the output index of the target node.
    pub fn set_input_post_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_post_solve_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_reconnectdist(mut self, val: f32) -> Self {
        self.params.insert(
            "reconnectdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reconnectdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reconnectdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minedgelen(mut self, val: f32) -> Self {
        self.params.insert(
            "minedgelen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minedgelen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minedgelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_maxedgelen(mut self, val: f32) -> Self {
        self.params.insert(
            "maxedgelen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxedgelen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxedgelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speedcap(mut self, val: f32) -> Self {
        self.params.insert(
            "speedcap".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_speedcap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speedcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activate(mut self, val: i32) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_bindgeo(mut self, val: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bindgeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindgeo".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dospeedcap(mut self, val: bool) -> Self {
        self.params.insert(
            "dospeedcap".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dospeedcap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dospeedcap".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solverperobject(mut self, val: bool) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solverperobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFilamentsolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "filamentsolver"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFileMode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFileCompresssims {
    NoCompression = 0,
    Blosc = 1,
    Gzip = 2,
}

#[derive(Debug, Clone)]
pub struct DopFile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFile {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to use when not reading from file"
    pub fn set_input_objects_to_use_when_not_reading_from_fil<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to use when not reading from file" and specifies the output index of the target node.
    pub fn set_input_objects_to_use_when_not_reading_from_fil_from<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_use_when_not_reading_from_fil_by_name<
        N: crate::core::types::HoudiniNode,
    >(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: DopFileMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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

    // --- Menu parameters ---
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_compresssims(mut self, val: DopFileCompresssims) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compresssims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchprefix(mut self, val: &str) -> Self {
        self.params.insert(
            "matchprefix".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_matchprefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchprefix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_takeownership(mut self, val: bool) -> Self {
        self.params.insert(
            "takeownership".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_takeownership_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "takeownership".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchbyname(mut self, val: bool) -> Self {
        self.params.insert(
            "matchbyname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_matchbyname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchbyname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "file"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFiledataMode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFiledataSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopFiledata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
    next_input_index: usize,
}

impl DopFiledata {
    pub const OUT_OUTPUT1: &'static str = "output1";

    pub fn new(name: &str) -> Self {
        Self {
            id: crate::core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
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
    pub fn set_input_at<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self.next_input_index += 1;
        self
    }

    pub fn add_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            self.next_input_index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self.next_input_index += 1;
        self
    }

    // --- Int parameters ---
    pub fn with_mode(mut self, val: DopFiledataMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
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
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopFiledataSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activationrules(mut self, val: i32) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_activationrules_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activationrules".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_newdataonly(mut self, val: bool) -> Self {
        self.params.insert(
            "newdataonly".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_newdataonly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newdataonly".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFiledata {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "filedata"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFlipconfigureobjectSurfaceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectSurfaceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectSurfaceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectVelGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectVelGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectVelGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    BlackBody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectCollisionGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectCollisionGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectCollisionGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectCollisionvelGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectCollisionvelGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectCollisionvelGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    BlackBody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectPressureGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectPressureGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectPressureGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectSourceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectSourceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectSourceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectDensityGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectDensityGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectDensityGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectViscosityGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectViscosityGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectViscosityGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectDivergenceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectDivergenceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipconfigureobjectDivergenceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone)]
pub struct DopFlipconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFlipconfigureobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_coldivsize(mut self, val: f32) -> Self {
        self.params.insert(
            "coldivsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_coldivsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "coldivsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidestreamerlen(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerlen(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_surface_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_surface_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "vel_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_vel_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_collision_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "collisionvel_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_collisionvel_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pressure_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pressure_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_source_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "density_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_density_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "viscosity_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_viscosity_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "divergence_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_divergence_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_surface_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_surface_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collision_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionvel_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionvel_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionweights_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionweights_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionweights_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionweights_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pressure_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pressure_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_source_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "viscosity_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_viscosity_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "density_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_density_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "divergence_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_divergence_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_vel_guidediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "vel_guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_vel_guidediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "collisionvel_guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_collisionvel_guidediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_surface_guideplane(mut self, val: DopFlipconfigureobjectSurfaceGuideplane) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevismode(
        mut self,
        val: DopFlipconfigureobjectSurfaceGuidevismode,
    ) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode(
        mut self,
        val: DopFlipconfigureobjectSurfaceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideplane(mut self, val: DopFlipconfigureobjectVelGuideplane) -> Self {
        self.params.insert(
            "vel_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevistype(mut self, val: DopFlipconfigureobjectVelGuidevistype) -> Self {
        self.params.insert(
            "vel_guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevismode(mut self, val: DopFlipconfigureobjectVelGuidevismode) -> Self {
        self.params.insert(
            "vel_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplane(
        mut self,
        val: DopFlipconfigureobjectCollisionGuideplane,
    ) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevismode(
        mut self,
        val: DopFlipconfigureobjectCollisionGuidevismode,
    ) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode(
        mut self,
        val: DopFlipconfigureobjectCollisionGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideplane(
        mut self,
        val: DopFlipconfigureobjectCollisionvelGuideplane,
    ) -> Self {
        self.params.insert(
            "collisionvel_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionvel_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidevistype(
        mut self,
        val: DopFlipconfigureobjectCollisionvelGuidevistype,
    ) -> Self {
        self.params.insert(
            "collisionvel_guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionvel_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidevismode(
        mut self,
        val: DopFlipconfigureobjectCollisionvelGuidevismode,
    ) -> Self {
        self.params.insert(
            "collisionvel_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionvel_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideplane(
        mut self,
        val: DopFlipconfigureobjectPressureGuideplane,
    ) -> Self {
        self.params.insert(
            "pressure_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pressure_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guidevismode(
        mut self,
        val: DopFlipconfigureobjectPressureGuidevismode,
    ) -> Self {
        self.params.insert(
            "pressure_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pressure_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guidevisdensitymode(
        mut self,
        val: DopFlipconfigureobjectPressureGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "pressure_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pressure_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplane(mut self, val: DopFlipconfigureobjectSourceGuideplane) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevismode(
        mut self,
        val: DopFlipconfigureobjectSourceGuidevismode,
    ) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevisdensitymode(
        mut self,
        val: DopFlipconfigureobjectSourceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideplane(mut self, val: DopFlipconfigureobjectDensityGuideplane) -> Self {
        self.params.insert(
            "density_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_density_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guidevismode(
        mut self,
        val: DopFlipconfigureobjectDensityGuidevismode,
    ) -> Self {
        self.params.insert(
            "density_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_density_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guidevisdensitymode(
        mut self,
        val: DopFlipconfigureobjectDensityGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "density_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_density_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideplane(
        mut self,
        val: DopFlipconfigureobjectViscosityGuideplane,
    ) -> Self {
        self.params.insert(
            "viscosity_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscosity_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guidevismode(
        mut self,
        val: DopFlipconfigureobjectViscosityGuidevismode,
    ) -> Self {
        self.params.insert(
            "viscosity_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscosity_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guidevisdensitymode(
        mut self,
        val: DopFlipconfigureobjectViscosityGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "viscosity_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscosity_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideplane(
        mut self,
        val: DopFlipconfigureobjectDivergenceGuideplane,
    ) -> Self {
        self.params.insert(
            "divergence_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_divergence_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guidevismode(
        mut self,
        val: DopFlipconfigureobjectDivergenceGuidevismode,
    ) -> Self {
        self.params.insert(
            "divergence_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_divergence_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guidevisdensitymode(
        mut self,
        val: DopFlipconfigureobjectDivergenceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "divergence_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_divergence_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_import_surface(mut self, val: &str) -> Self {
        self.params.insert(
            "import_surface".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_import_surface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "import_surface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_import_velocity(mut self, val: &str) -> Self {
        self.params.insert(
            "import_velocity".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_import_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "import_velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_adddivergence(mut self, val: bool) -> Self {
        self.params.insert(
            "adddivergence".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adddivergence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adddivergence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesoppath(mut self, val: bool) -> Self {
        self.params.insert(
            "usesoppath".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesoppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closedends(mut self, val: bool) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closedends_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeypos(mut self, val: bool) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeyneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionweights_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionweights_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionweights_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionweights_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "density_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusebox(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideuseboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideoverridediv(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidebarbs(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidepercomp(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusestreamers(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideusebox(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideusebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideuseboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideusesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideoverridediv(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidebarbs(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidepercomp(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideusestreamers(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "density_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "density_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "density_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFlipconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipconfigureobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFlipobjectSurfacetype {
    SurfaceSop = 0,
    ParticleField = 1,
    File = 2,
    FetchData = 3,
    NarrowBand = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectInittype {
    Grid = 0,
    Tetrahedral = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectVisprim {
    Spheres = 0,
    Sprites = 1,
    Grain = 2,
    Particles = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectGuidevistype {
    None = 0,
    Speed = 1,
    Direction = 2,
    Value = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectGuidevismode {
    Ramp = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectSurfaceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectSurfaceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectSurfaceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectVelGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectVelGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectVelGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    BlackBody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectCollisionGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectCollisionGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectCollisionGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectCollisionvelGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectCollisionvelGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectCollisionvelGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    BlackBody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectPressureGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectPressureGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectPressureGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectSourceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectSourceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectSourceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectDensityGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectDensityGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectDensityGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectViscosityGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectViscosityGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectViscosityGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectDivergenceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectDivergenceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipobjectDivergenceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone)]
pub struct DopFlipobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFlipobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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

    // --- Float parameters ---
    pub fn with_particlesep(mut self, val: f32) -> Self {
        self.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_particlesep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "particlesep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_radiusscale(mut self, val: f32) -> Self {
        self.params.insert(
            "radiusscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radiusscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radiusscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gridscale(mut self, val: f32) -> Self {
        self.params.insert(
            "gridscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gridscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gridscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionsep(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionsep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionsep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionsep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterscale(mut self, val: f32) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_jitterscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitterscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visscale(mut self, val: f32) -> Self {
        self.params.insert(
            "visscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_visscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidestreamerlen(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vel_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerlen(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvel_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "pressure_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pressure_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "density_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "divergence_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergence_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounceforward(mut self, val: f32) -> Self {
        self.params.insert(
            "bounceforward".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounceforward_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounceforward".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamicfriction(mut self, val: f32) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamicfriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_temperature(mut self, val: f32) -> Self {
        self.params.insert(
            "temperature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_temperature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "temperature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumeoffset(mut self, val: f32) -> Self {
        self.params.insert(
            "volumeoffset".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumeoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumeoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_surface_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "vel_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_vel_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_collision_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "collisionvel_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_collisionvel_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pressure_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pressure_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_source_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "density_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_density_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "viscosity_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_viscosity_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "divergence_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_divergence_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_velocity(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "viscolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_viscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_surface_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collision_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collisionvel_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collisionvel_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pressure_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pressure_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_source_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "density_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_density_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "viscosity_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_viscosity_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "divergence_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_divergence_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_createframe(mut self, val: i32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_jitterseed(mut self, val: i32) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_jitterseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "jitterseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_vel_guidediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "vel_guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_vel_guidediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "collisionvel_guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_collisionvel_guidediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_surfacetype(mut self, val: DopFlipobjectSurfacetype) -> Self {
        self.params.insert(
            "surfacetype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surfacetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacetype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_inittype(mut self, val: DopFlipobjectInittype) -> Self {
        self.params.insert(
            "inittype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_inittype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inittype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visprim(mut self, val: DopFlipobjectVisprim) -> Self {
        self.params.insert(
            "visprim".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_visprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visprim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidevistype(mut self, val: DopFlipobjectGuidevistype) -> Self {
        self.params.insert(
            "guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidevismode(mut self, val: DopFlipobjectGuidevismode) -> Self {
        self.params.insert(
            "guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplane(mut self, val: DopFlipobjectSurfaceGuideplane) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevismode(mut self, val: DopFlipobjectSurfaceGuidevismode) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode(
        mut self,
        val: DopFlipobjectSurfaceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideplane(mut self, val: DopFlipobjectVelGuideplane) -> Self {
        self.params.insert(
            "vel_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevistype(mut self, val: DopFlipobjectVelGuidevistype) -> Self {
        self.params.insert(
            "vel_guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidevismode(mut self, val: DopFlipobjectVelGuidevismode) -> Self {
        self.params.insert(
            "vel_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vel_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplane(mut self, val: DopFlipobjectCollisionGuideplane) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevismode(mut self, val: DopFlipobjectCollisionGuidevismode) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode(
        mut self,
        val: DopFlipobjectCollisionGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideplane(
        mut self,
        val: DopFlipobjectCollisionvelGuideplane,
    ) -> Self {
        self.params.insert(
            "collisionvel_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionvel_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidevistype(
        mut self,
        val: DopFlipobjectCollisionvelGuidevistype,
    ) -> Self {
        self.params.insert(
            "collisionvel_guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionvel_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidevismode(
        mut self,
        val: DopFlipobjectCollisionvelGuidevismode,
    ) -> Self {
        self.params.insert(
            "collisionvel_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionvel_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideplane(mut self, val: DopFlipobjectPressureGuideplane) -> Self {
        self.params.insert(
            "pressure_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pressure_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guidevismode(mut self, val: DopFlipobjectPressureGuidevismode) -> Self {
        self.params.insert(
            "pressure_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pressure_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guidevisdensitymode(
        mut self,
        val: DopFlipobjectPressureGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "pressure_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pressure_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplane(mut self, val: DopFlipobjectSourceGuideplane) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevismode(mut self, val: DopFlipobjectSourceGuidevismode) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevisdensitymode(
        mut self,
        val: DopFlipobjectSourceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideplane(mut self, val: DopFlipobjectDensityGuideplane) -> Self {
        self.params.insert(
            "density_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_density_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guidevismode(mut self, val: DopFlipobjectDensityGuidevismode) -> Self {
        self.params.insert(
            "density_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_density_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guidevisdensitymode(
        mut self,
        val: DopFlipobjectDensityGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "density_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_density_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideplane(mut self, val: DopFlipobjectViscosityGuideplane) -> Self {
        self.params.insert(
            "viscosity_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscosity_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guidevismode(mut self, val: DopFlipobjectViscosityGuidevismode) -> Self {
        self.params.insert(
            "viscosity_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscosity_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guidevisdensitymode(
        mut self,
        val: DopFlipobjectViscosityGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "viscosity_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscosity_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideplane(mut self, val: DopFlipobjectDivergenceGuideplane) -> Self {
        self.params.insert(
            "divergence_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_divergence_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guidevismode(
        mut self,
        val: DopFlipobjectDivergenceGuidevismode,
    ) -> Self {
        self.params.insert(
            "divergence_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_divergence_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guidevisdensitymode(
        mut self,
        val: DopFlipobjectDivergenceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "divergence_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_divergence_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Ramp parameters ---
    pub fn with_guidevisramp(mut self, val: Vec<crate::core::types::RampPoint>) -> Self {
        self.params.insert(
            "guidevisramp".to_string(),
            crate::core::types::ParamValue::Ramp(val),
        );
        self
    }
    pub fn with_guidevisramp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevisramp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_objname(mut self, val: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_objname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_import_nbsurface(mut self, val: &str) -> Self {
        self.params.insert(
            "import_nbsurface".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_import_nbsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "import_nbsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_import_nbvelocity(mut self, val: &str) -> Self {
        self.params.insert(
            "import_nbvelocity".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_import_nbvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "import_nbvelocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fluid_geometry_file(mut self, val: &str) -> Self {
        self.params.insert(
            "fluid_geometry_file".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fluid_geometry_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fluid_geometry_file".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srcobject(mut self, val: &str) -> Self {
        self.params.insert(
            "srcobject".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srcobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcobject".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srcdataname(mut self, val: &str) -> Self {
        self.params.insert(
            "srcdataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_srcdataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srcdataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spriteimage(mut self, val: &str) -> Self {
        self.params.insert(
            "spriteimage".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_spriteimage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spriteimage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "guideattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guideattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_overridecollisionsep(mut self, val: bool) -> Self {
        self.params.insert(
            "overridecollisionsep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overridecollisionsep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overridecollisionsep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closedends(mut self, val: bool) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closedends_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeypos(mut self, val: bool) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeyneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowcaching(mut self, val: bool) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowcaching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initfluidattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "initfluidattribs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initfluidattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initfluidattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initvel(mut self, val: bool) -> Self {
        self.params.insert(
            "initvel".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initvel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addviscosity(mut self, val: bool) -> Self {
        self.params.insert(
            "addviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addviscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addtemperature(mut self, val: bool) -> Self {
        self.params.insert(
            "addtemperature".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addtemperature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addtemperature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adddivergence(mut self, val: bool) -> Self {
        self.params.insert(
            "adddivergence".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adddivergence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adddivergence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "density_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidedetectrange(mut self, val: bool) -> Self {
        self.params.insert(
            "guidedetectrange".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guidedetectrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidedetectrange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusebox(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideuseboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideoverridediv(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidebarbs(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guidepercomp(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideusestreamers(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vel_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "vel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vel_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideusebox(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideusebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideuseboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideusesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideoverridediv(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidebarbs(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guidepercomp(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideusestreamers(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvel_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "collisionvel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collisionvel_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvel_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pressure_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "pressure_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pressure_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressure_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "density_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "density_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "density_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_density_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "density_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_density_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergence_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "divergence_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_divergence_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergence_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepointvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "usepointvelocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepointvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepointvelocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesdfvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "usesdfvelocity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesdfvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesdfvelocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFlipobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFlipsolverUnderresolved {
    NoDetection = 0,
    DetectOnly = 1,
    TreatAsBallistic = 2,
    UseExtrapolatedVelocity = 3,
    Kill = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverCollision {
    None = 0,
    Particle = 1,
    MoveOutsideCollision = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverDropletbehavior {
    BlendWithFluid = 0,
    KillOnDetection = 1,
    KillAtFluid = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverVorticitymix {
    Copy = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Maximum = 5,
    Minimum = 6,
    Average = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverVeltransfer {
    /// FLIP (Splashy)
    FlipSplashy = 0,
    /// APIC (Swirly)
    ApicSwirly = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverUpdatesurface {
    None = 0,
    Advect = 1,
    Rebuild = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverUpdatevel {
    None = 0,
    Advect = 1,
    Rebuild = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverVeltype {
    Rigid = 0,
    Point = 1,
    Volume = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverFractionmethod {
    CollisionSupersampling = 0,
    VoxelFaceArea = 1,
    Tetrahedral = 2,
    None = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverViscositymix {
    Copy = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Maximum = 5,
    Minimum = 6,
    Average = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverViscosityprecision {
    /// Float 32-bit
    Float32MinusBit = 0,
    /// Float 64-bit
    Float64MinusBit = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverDensitymix {
    Copy = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Maximum = 5,
    Minimum = 6,
    Average = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverDivergencemix {
    Copy = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Maximum = 5,
    Minimum = 6,
    Average = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverStfilter {
    Box = 0,
    Cone = 1,
    Gaussian = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFlipsolverExtrapmode {
    Normal = 0,
    /// Fast-moving Colliders
    FastMinusMovingColliders = 1,
}

#[derive(Debug, Clone)]
pub struct DopFlipsolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFlipsolver {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Fluid to Solve"
    pub fn set_input_fluid_to_solve<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Fluid to Solve" and specifies the output index of the target node.
    pub fn set_input_fluid_to_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_fluid_to_solve_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Particle Velocity"
    pub fn set_input_particle_velocity<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Particle Velocity" and specifies the output index of the target node.
    pub fn set_input_particle_velocity_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_particle_velocity_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: "Volume Velocity"
    pub fn set_input_volume_velocity<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "Volume Velocity" and specifies the output index of the target node.
    pub fn set_input_volume_velocity_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_volume_velocity_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 3: "Sourcing (post-solve)"
    pub fn set_input_sourcing_post_solve<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "Sourcing (post-solve)" and specifies the output index of the target node.
    pub fn set_input_sourcing_post_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_sourcing_post_solve_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cflcond(mut self, val: f32) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cflcond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partcflcond(mut self, val: f32) -> Self {
        self.params.insert(
            "partcflcond".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partcflcond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partcflcond".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversampling(mut self, val: f32) -> Self {
        self.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversampling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oversampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversamplingbandwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_oversamplingbandwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oversamplingbandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_birththreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "birththreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_birththreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "birththreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_deaththreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "deaththreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_deaththreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "deaththreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "seed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsepamount(mut self, val: f32) -> Self {
        self.params.insert(
            "partsepamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partsepamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partsepamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsepscale(mut self, val: f32) -> Self {
        self.params.insert(
            "partsepscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_partsepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partsepscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletmindensity(mut self, val: f32) -> Self {
        self.params.insert(
            "dropletmindensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dropletmindensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dropletmindensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletmaxdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "dropletmaxdensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dropletmaxdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dropletmaxdensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletvelblend(mut self, val: f32) -> Self {
        self.params.insert(
            "dropletvelblend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dropletvelblend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dropletvelblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorticitypreserve(mut self, val: f32) -> Self {
        self.params.insert(
            "vorticitypreserve".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vorticitypreserve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vorticitypreserve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorticityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "vorticityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vorticityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vorticityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_volumeamount(mut self, val: f32) -> Self {
        self.params.insert(
            "volumeamount".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_volumeamount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumeamount".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_smoothing(mut self, val: f32) -> Self {
        self.params.insert(
            "smoothing".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_smoothing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smoothing".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waterline(mut self, val: f32) -> Self {
        self.params.insert(
            "waterline".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_waterline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvelscale(mut self, val: f32) -> Self {
        self.params.insert(
            "collisionvelscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collisionvelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrapdist(mut self, val: f32) -> Self {
        self.params.insert(
            "extrapdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_extrapdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrapdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transparency(mut self, val: f32) -> Self {
        self.params.insert(
            "transparency".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_transparency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transparency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickscale(mut self, val: f32) -> Self {
        self.params.insert(
            "stickscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickdist(mut self, val: f32) -> Self {
        self.params.insert(
            "stickdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickcells(mut self, val: f32) -> Self {
        self.params.insert(
            "stickcells".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickcells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickcells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickbias(mut self, val: f32) -> Self {
        self.params.insert(
            "stickbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stickbias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sticknormalscale(mut self, val: f32) -> Self {
        self.params.insert(
            "sticknormalscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sticknormalscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sticknormalscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sticktangentscale(mut self, val: f32) -> Self {
        self.params.insert(
            "sticktangentscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sticktangentscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sticktangentscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slipscale(mut self, val: f32) -> Self {
        self.params.insert(
            "slipscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_slipscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slipscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densityscale(mut self, val: f32) -> Self {
        self.params.insert(
            "densityscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_densityscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densityscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergencescale(mut self, val: f32) -> Self {
        self.params.insert(
            "divergencescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divergencescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergencescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacetension(mut self, val: f32) -> Self {
        self.params.insert(
            "surfacetension".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfacetension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacetension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stblurradius(mut self, val: f32) -> Self {
        self.params.insert(
            "stblurradius".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_stblurradius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stblurradius".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spatialscale(mut self, val: f32) -> Self {
        self.params.insert(
            "spatialscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spatialscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatialscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_massscale(mut self, val: f32) -> Self {
        self.params.insert(
            "massscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_massscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "massscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_feedbackscale(mut self, val: f32) -> Self {
        self.params.insert(
            "feedbackscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_feedbackscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "feedbackscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velextrapolatemaxcells(mut self, val: f32) -> Self {
        self.params.insert(
            "velextrapolatemaxcells".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velextrapolatemaxcells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velextrapolatemaxcells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_narrowbandwidth(mut self, val: f32) -> Self {
        self.params.insert(
            "narrowbandwidth".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_narrowbandwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "narrowbandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_vislimitcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vislimitcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vislimitcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vislimitcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limit_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "limit_size".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_limit_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limit_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limit_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "limit_t".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_limit_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limit_t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_waterlinedirection(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "waterlinedirection".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_waterlinedirection_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "waterlinedirection".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary_lowerpadding(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "boundary_lowerpadding".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_boundary_lowerpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundary_lowerpadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary_upperpadding(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "boundary_upperpadding".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_boundary_upperpadding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundary_upperpadding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_minimumsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partpervoxel(mut self, val: i32) -> Self {
        self.params.insert(
            "partpervoxel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_partpervoxel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partpervoxel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsepiter(mut self, val: i32) -> Self {
        self.params.insert(
            "partsepiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_partsepiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partsepiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rest_framedelay(mut self, val: i32) -> Self {
        self.params.insert(
            "rest_framedelay".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rest_framedelay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rest_framedelay".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rest_frameoffset(mut self, val: i32) -> Self {
        self.params.insert(
            "rest_frameoffset".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rest_frameoffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rest_frameoffset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numsupersamples(mut self, val: i32) -> Self {
        self.params.insert(
            "numsupersamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numsupersamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numsupersamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minairregionsize(mut self, val: i32) -> Self {
        self.params.insert(
            "minairregionsize".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minairregionsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minairregionsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_port(mut self, val: i32) -> Self {
        self.params
            .insert("port".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_port_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "port".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice(mut self, val: i32) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numslice(mut self, val: i32) -> Self {
        self.params.insert(
            "numslice".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_underresolved(mut self, val: DopFlipsolverUnderresolved) -> Self {
        self.params.insert(
            "underresolved".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_underresolved_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "underresolved".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision(mut self, val: DopFlipsolverCollision) -> Self {
        self.params.insert(
            "collision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dropletbehavior(mut self, val: DopFlipsolverDropletbehavior) -> Self {
        self.params.insert(
            "dropletbehavior".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_dropletbehavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dropletbehavior".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vorticitymix(mut self, val: DopFlipsolverVorticitymix) -> Self {
        self.params.insert(
            "vorticitymix".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vorticitymix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vorticitymix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veltransfer(mut self, val: DopFlipsolverVeltransfer) -> Self {
        self.params.insert(
            "veltransfer".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_veltransfer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veltransfer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updatesurface(mut self, val: DopFlipsolverUpdatesurface) -> Self {
        self.params.insert(
            "updatesurface".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_updatesurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updatesurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_updatevel(mut self, val: DopFlipsolverUpdatevel) -> Self {
        self.params.insert(
            "updatevel".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_updatevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "updatevel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veltype(mut self, val: DopFlipsolverVeltype) -> Self {
        self.params.insert(
            "veltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_veltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fractionmethod(mut self, val: DopFlipsolverFractionmethod) -> Self {
        self.params.insert(
            "fractionmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fractionmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractionmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscositymix(mut self, val: DopFlipsolverViscositymix) -> Self {
        self.params.insert(
            "viscositymix".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscositymix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscositymix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosityprecision(mut self, val: DopFlipsolverViscosityprecision) -> Self {
        self.params.insert(
            "viscosityprecision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_viscosityprecision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosityprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densitymix(mut self, val: DopFlipsolverDensitymix) -> Self {
        self.params.insert(
            "densitymix".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_densitymix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densitymix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergencemix(mut self, val: DopFlipsolverDivergencemix) -> Self {
        self.params.insert(
            "divergencemix".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_divergencemix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergencemix".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stfilter(mut self, val: DopFlipsolverStfilter) -> Self {
        self.params.insert(
            "stfilter".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_stfilter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stfilter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extrapmode(mut self, val: DopFlipsolverExtrapmode) -> Self {
        self.params.insert(
            "extrapmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_extrapmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extrapmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_volumeoverrideattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "volumeoverrideattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_volumeoverrideattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumeoverrideattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_delattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "delattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_delattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_interpattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "interpattribs".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_interpattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "interpattribs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary_surface(mut self, val: &str) -> Self {
        self.params.insert(
            "boundary_surface".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundary_surface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundary_surface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_boundary_velocity(mut self, val: &str) -> Self {
        self.params.insert(
            "boundary_velocity".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_boundary_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "boundary_velocity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickcontrolfield(mut self, val: &str) -> Self {
        self.params.insert(
            "stickcontrolfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stickcontrolfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickcontrolfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosityattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "viscosityattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_viscosityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosityattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slipcontrolfield(mut self, val: &str) -> Self {
        self.params.insert(
            "slipcontrolfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_slipcontrolfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slipcontrolfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_densityattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "densityattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_densityattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "densityattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divergenceattrib(mut self, val: &str) -> Self {
        self.params.insert(
            "divergenceattrib".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_divergenceattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divergenceattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stblurmaskfield(mut self, val: &str) -> Self {
        self.params.insert(
            "stblurmaskfield".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stblurmaskfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stblurmaskfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionsurfacename(mut self, val: &str) -> Self {
        self.params.insert(
            "collisionsurfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collisionsurfacename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionsurfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionweightsname(mut self, val: &str) -> Self {
        self.params.insert(
            "collisionweightsname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collisionweightsname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionweightsname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionvelname(mut self, val: &str) -> Self {
        self.params.insert(
            "collisionvelname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_collisionvelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionvelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solversurfacename(mut self, val: &str) -> Self {
        self.params.insert(
            "solversurfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_solversurfacename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solversurfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourcesurfacename(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesurfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sourcesurfacename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesurfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sinksurfacename(mut self, val: &str) -> Self {
        self.params.insert(
            "sinksurfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_sinksurfacename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sinksurfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidingsurfacename(mut self, val: &str) -> Self {
        self.params.insert(
            "guidingsurfacename".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidingsurfacename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidingsurfacename".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guidingvelname(mut self, val: &str) -> Self {
        self.params.insert(
            "guidingvelname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_guidingvelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidingvelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_narrowband_pairsop(mut self, val: &str) -> Self {
        self.params.insert(
            "narrowband_pairsop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_narrowband_pairsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "narrowband_pairsop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_initvolumessop(mut self, val: &str) -> Self {
        self.params.insert(
            "initvolumessop".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_initvolumessop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initvolumessop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_attributepair_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("attributepair{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_attributepair_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("attributepair{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fieldpair_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("fieldpair{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fieldpair_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("fieldpair{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_address(mut self, val: &str) -> Self {
        self.params.insert(
            "address".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_address_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "address".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_quantize(mut self, val: bool) -> Self {
        self.params.insert(
            "quantize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doforces(mut self, val: bool) -> Self {
        self.params.insert(
            "doforces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doforces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doforces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablevolumeattrib(mut self, val: bool) -> Self {
        self.params.insert(
            "enablevolumeattrib".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablevolumeattrib_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablevolumeattrib".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_killunmoveable(mut self, val: bool) -> Self {
        self.params.insert(
            "killunmoveable".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_killunmoveable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "killunmoveable".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tankcollision(mut self, val: bool) -> Self {
        self.params.insert(
            "tankcollision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tankcollision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tankcollision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_killoutside(mut self, val: bool) -> Self {
        self.params.insert(
            "killoutside".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_killoutside_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "killoutside".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usephysparms(mut self, val: bool) -> Self {
        self.params.insert(
            "usephysparms".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usephysparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usephysparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doid(mut self, val: bool) -> Self {
        self.params.insert(
            "doid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dorelativedensity(mut self, val: bool) -> Self {
        self.params.insert(
            "dorelativedensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dorelativedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dorelativedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doage(mut self, val: bool) -> Self {
        self.params.insert(
            "doage".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doage".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doreapparticles(mut self, val: bool) -> Self {
        self.params.insert(
            "doreapparticles".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doreapparticles_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doreapparticles".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reseedsinglepass(mut self, val: bool) -> Self {
        self.params.insert(
            "reseedsinglepass".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reseedsinglepass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reseedsinglepass".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onlysourceseeding(mut self, val: bool) -> Self {
        self.params.insert(
            "onlysourceseeding".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_onlysourceseeding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "onlysourceseeding".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reseed(mut self, val: bool) -> Self {
        self.params.insert(
            "reseed".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reseed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_oversamplebounds(mut self, val: bool) -> Self {
        self.params.insert(
            "oversamplebounds".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_oversamplebounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oversamplebounds".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_partsep(mut self, val: bool) -> Self {
        self.params.insert(
            "partsep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_partsep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "partsep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodroplets(mut self, val: bool) -> Self {
        self.params.insert(
            "dodroplets".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodroplets_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodroplets".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dovorticity(mut self, val: bool) -> Self {
        self.params.insert(
            "dovorticity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dovorticity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dovorticity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_rest(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_rest".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_rest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_rest".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dual_rest_attributes(mut self, val: bool) -> Self {
        self.params.insert(
            "dual_rest_attributes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dual_rest_attributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dual_rest_attributes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfacereinit(mut self, val: bool) -> Self {
        self.params.insert(
            "surfacereinit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surfacereinit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfacereinit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vislimit(mut self, val: bool) -> Self {
        self.params.insert(
            "vislimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vislimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vislimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamicresize(mut self, val: bool) -> Self {
        self.params.insert(
            "dynamicresize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dynamicresize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamicresize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fillnewvolume(mut self, val: bool) -> Self {
        self.params.insert(
            "fillnewvolume".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fillnewvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fillnewvolume".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usewaterline(mut self, val: bool) -> Self {
        self.params.insert(
            "usewaterline".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usewaterline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usewaterline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_visualwaterline(mut self, val: bool) -> Self {
        self.params.insert(
            "visualwaterline".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_visualwaterline_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visualwaterline".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useboundarylayer(mut self, val: bool) -> Self {
        self.params.insert(
            "useboundarylayer".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useboundarylayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useboundarylayer".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applybound(mut self, val: bool) -> Self {
        self.params.insert(
            "applybound".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applybound_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applybound".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransparency(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransparency".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransparency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransparency".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enablestick(mut self, val: bool) -> Self {
        self.params.insert(
            "enablestick".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablestick_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablestick".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickusemaxdist(mut self, val: bool) -> Self {
        self.params.insert(
            "stickusemaxdist".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickusemaxdist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickusemaxdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickusemaxcells(mut self, val: bool) -> Self {
        self.params.insert(
            "stickusemaxcells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickusemaxcells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickusemaxcells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickusecontrolfield(mut self, val: bool) -> Self {
        self.params.insert(
            "stickusecontrolfield".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickusecontrolfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickusecontrolfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity(mut self, val: bool) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_viscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useadaptiveviscosity(mut self, val: bool) -> Self {
        self.params.insert(
            "useadaptiveviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useadaptiveviscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useadaptiveviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doviscosity(mut self, val: bool) -> Self {
        self.params.insert(
            "doviscosity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doviscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doviscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enableslip(mut self, val: bool) -> Self {
        self.params.insert(
            "enableslip".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableslip_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableslip".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slipusecontrolfield(mut self, val: bool) -> Self {
        self.params.insert(
            "slipusecontrolfield".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_slipusecontrolfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slipusecontrolfield".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodensity(mut self, val: bool) -> Self {
        self.params.insert(
            "dodensity".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_doincompressibleair(mut self, val: bool) -> Self {
        self.params.insert(
            "doincompressibleair".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doincompressibleair_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doincompressibleair".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_applycollisionstoair(mut self, val: bool) -> Self {
        self.params.insert(
            "applycollisionstoair".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_applycollisionstoair_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "applycollisionstoair".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodivergence(mut self, val: bool) -> Self {
        self.params.insert(
            "dodivergence".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodivergence_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodivergence".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dosurfacetension(mut self, val: bool) -> Self {
        self.params.insert(
            "dosurfacetension".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dosurfacetension_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dosurfacetension".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_docurvatureblur(mut self, val: bool) -> Self {
        self.params.insert(
            "docurvatureblur".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_docurvatureblur_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "docurvatureblur".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_autoextrapolatemaxcells(mut self, val: bool) -> Self {
        self.params.insert(
            "autoextrapolatemaxcells".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autoextrapolatemaxcells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autoextrapolatemaxcells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usepreconditioner(mut self, val: bool) -> Self {
        self.params.insert(
            "usepreconditioner".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepreconditioner_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepreconditioner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usemgpreconditioner(mut self, val: bool) -> Self {
        self.params.insert(
            "usemgpreconditioner".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemgpreconditioner_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemgpreconditioner".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usewarmstart(mut self, val: bool) -> Self {
        self.params.insert(
            "usewarmstart".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usewarmstart_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usewarmstart".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_useadaptivepressure(mut self, val: bool) -> Self {
        self.params.insert(
            "useadaptivepressure".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useadaptivepressure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useadaptivepressure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opencl(mut self, val: bool) -> Self {
        self.params.insert(
            "opencl".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_opencl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opencl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_donarrowband(mut self, val: bool) -> Self {
        self.params.insert(
            "donarrowband".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_donarrowband_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "donarrowband".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodistributedsolve(mut self, val: bool) -> Self {
        self.params.insert(
            "dodistributedsolve".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodistributedsolve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodistributedsolve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFlipsolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "flipsolver"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFluidconfigureobjectVoxelplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSurfaceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSurfaceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSurfaceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectVelocityGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectVelocityGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectVelocityGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    BlackBody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectCollisionGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectCollisionGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectCollisionGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSourceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSourceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSourceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSinkGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSinkGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectSinkGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectPumpGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectPumpGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectPumpGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidconfigureobjectBorder {
    Constant = 0,
    Repeat = 1,
    Streak = 2,
}

#[derive(Debug, Clone)]
pub struct DopFluidconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFluidconfigureobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidestreamerlen(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamicfriction(mut self, val: f32) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamicfriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_temperature(mut self, val: f32) -> Self {
        self.params.insert(
            "temperature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_temperature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "temperature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_surface_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_surface_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "velocity_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_velocity_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_collision_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_source_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sink_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sink_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pump_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pump_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_surface_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "velocity_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_velocity_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collision_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_source_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sink_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sink_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pump_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pump_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice(mut self, val: i32) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_div(mut self, val: [i32; 3]) -> Self {
        self.params
            .insert("div".to_string(), crate::core::types::ParamValue::Int3(val));
        self
    }
    pub fn with_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "velocity_guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_velocity_guidediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slicediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_slicediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceoverlapneg(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlapneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceoverlappos(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlappos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_voxelplane(mut self, val: DopFluidconfigureobjectVoxelplane) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voxelplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformvoxels(mut self, val: DopFluidconfigureobjectUniformvoxels) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplane(
        mut self,
        val: DopFluidconfigureobjectSurfaceGuideplane,
    ) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevismode(
        mut self,
        val: DopFluidconfigureobjectSurfaceGuidevismode,
    ) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode(
        mut self,
        val: DopFluidconfigureobjectSurfaceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideplane(
        mut self,
        val: DopFluidconfigureobjectVelocityGuideplane,
    ) -> Self {
        self.params.insert(
            "velocity_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velocity_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidevistype(
        mut self,
        val: DopFluidconfigureobjectVelocityGuidevistype,
    ) -> Self {
        self.params.insert(
            "velocity_guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velocity_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidevismode(
        mut self,
        val: DopFluidconfigureobjectVelocityGuidevismode,
    ) -> Self {
        self.params.insert(
            "velocity_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velocity_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplane(
        mut self,
        val: DopFluidconfigureobjectCollisionGuideplane,
    ) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevismode(
        mut self,
        val: DopFluidconfigureobjectCollisionGuidevismode,
    ) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode(
        mut self,
        val: DopFluidconfigureobjectCollisionGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplane(mut self, val: DopFluidconfigureobjectSourceGuideplane) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevismode(
        mut self,
        val: DopFluidconfigureobjectSourceGuidevismode,
    ) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevisdensitymode(
        mut self,
        val: DopFluidconfigureobjectSourceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideplane(mut self, val: DopFluidconfigureobjectSinkGuideplane) -> Self {
        self.params.insert(
            "sink_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sink_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guidevismode(mut self, val: DopFluidconfigureobjectSinkGuidevismode) -> Self {
        self.params.insert(
            "sink_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sink_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guidevisdensitymode(
        mut self,
        val: DopFluidconfigureobjectSinkGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "sink_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sink_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideplane(mut self, val: DopFluidconfigureobjectPumpGuideplane) -> Self {
        self.params.insert(
            "pump_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pump_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guidevismode(mut self, val: DopFluidconfigureobjectPumpGuidevismode) -> Self {
        self.params.insert(
            "pump_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pump_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guidevisdensitymode(
        mut self,
        val: DopFluidconfigureobjectPumpGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "pump_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pump_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_border(mut self, val: DopFluidconfigureobjectBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_surface_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "surface_soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_surface_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "velocity_soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocity_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionpath(mut self, val: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_positionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_twod(mut self, val: bool) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_show_surfels_display(mut self, val: bool) -> Self {
        self.params.insert(
            "show_surfels_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_surfels_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_surfels_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideusebox(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideusebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideuseboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideusesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideoverridediv(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidebarbs(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidepercomp(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideusestreamers(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closedends(mut self, val: bool) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closedends_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeyneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeypos(mut self, val: bool) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFluidconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fluidconfigureobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFluidforceSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidforceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopFluidforce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFluidforce {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_objects_to_be_processed_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Input 1"
    pub fn set_input_input_1<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_input_1_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_forcescale(mut self, val: f32) -> Self {
        self.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_guideforcescale(mut self, val: f32) -> Self {
        self.params.insert(
            "guideforcescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guideforcescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideforcescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guideforcecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guideforcecolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guideforcecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideforcecolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_defaultparmop(mut self, val: DopFluidforceDefaultparmop) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_defaultparmop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultparmop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopFluidforceSharedata) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_sharedata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sharedata".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_activation(mut self, val: i32) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_activation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "activation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_samplemode(mut self, val: DopFluidforceSamplemode) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_fluid(mut self, val: &str) -> Self {
        self.params.insert(
            "fluid".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fluid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fluid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFluidforce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fluidforce"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFluidobjectVoxelplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSurfaceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSurfaceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSurfaceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectVelocityGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectVelocityGuidevistype {
    None = 0,
    Distance = 1,
    Speed = 2,
    Origin = 3,
    Direction = 4,
    Value = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectVelocityGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    BlackBody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectCollisionGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectCollisionGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectCollisionGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSourceGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSourceGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSourceGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSinkGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSinkGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectSinkGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectPumpGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectPumpGuidevismode {
    NoMapping = 0,
    /// Infra-Red
    InfraMinusRed = 1,
    WhiteToRed = 2,
    Grayscale = 3,
    BlackBody = 4,
    /// Two-Tone
    TwoMinusTone = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectPumpGuidevisdensitymode {
    NoMapping = 0,
    Increasing = 1,
    Decreasing = 2,
    Hill = 3,
    Valley = 4,
    Step = 5,
    Square = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidobjectBorder {
    Constant = 0,
    Repeat = 1,
    Streak = 2,
}

#[derive(Debug, Clone)]
pub struct DopFluidobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFluidobject {
    pub const OUT_OUTPUT1: &'static str = "output1";

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

    // --- Float parameters ---
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divsize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surface_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidescale(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidestreamerlen(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidestreamerlen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidestreamerlen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidestreamerminspeed(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidestreamerminspeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidestreamerminspeed".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidevisscale(mut self, val: f32) -> Self {
        self.params.insert(
            "velocity_guidevisscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velocity_guidevisscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidevisscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_collision_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_source_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "sink_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sink_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guidesmokedensity(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guidesmokedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guidesmokedensity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guideplaneval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideplaneval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderangecenter(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guiderangecenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderangecenter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderangesize(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guiderangesize".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guiderangesize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderangesize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideiso(mut self, val: f32) -> Self {
        self.params.insert(
            "pump_guideiso".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pump_guideiso_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideiso".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bounce(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dynamicfriction(mut self, val: f32) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamicfriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_temperature(mut self, val: f32) -> Self {
        self.params.insert(
            "temperature".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_temperature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "temperature".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_surface_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_surface_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "velocity_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_velocity_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_collision_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_source_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sink_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sink_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "pump_guiderange".to_string(),
            crate::core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_pump_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderange".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_surface_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "velocity_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_velocity_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_collision_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_source_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sink_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sink_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pump_color".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pump_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_color".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_createframe(mut self, val: i32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice(mut self, val: i32) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int3 parameters ---
    pub fn with_div(mut self, val: [i32; 3]) -> Self {
        self.params
            .insert("div".to_string(), crate::core::types::ParamValue::Int3(val));
        self
    }
    pub fn with_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "div".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "velocity_guidediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_velocity_guidediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slicediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_slicediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceoverlapneg(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlapneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sliceoverlappos(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlappos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_voxelplane(mut self, val: DopFluidobjectVoxelplane) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voxelplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniformvoxels(mut self, val: DopFluidobjectUniformvoxels) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideplane(mut self, val: DopFluidobjectSurfaceGuideplane) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevismode(mut self, val: DopFluidobjectSurfaceGuidevismode) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode(
        mut self,
        val: DopFluidobjectSurfaceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surface_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideplane(mut self, val: DopFluidobjectVelocityGuideplane) -> Self {
        self.params.insert(
            "velocity_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velocity_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidevistype(mut self, val: DopFluidobjectVelocityGuidevistype) -> Self {
        self.params.insert(
            "velocity_guidevistype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velocity_guidevistype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidevistype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidevismode(mut self, val: DopFluidobjectVelocityGuidevismode) -> Self {
        self.params.insert(
            "velocity_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velocity_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideplane(mut self, val: DopFluidobjectCollisionGuideplane) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevismode(mut self, val: DopFluidobjectCollisionGuidevismode) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode(
        mut self,
        val: DopFluidobjectCollisionGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collision_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideplane(mut self, val: DopFluidobjectSourceGuideplane) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevismode(mut self, val: DopFluidobjectSourceGuidevismode) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guidevisdensitymode(
        mut self,
        val: DopFluidobjectSourceGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideplane(mut self, val: DopFluidobjectSinkGuideplane) -> Self {
        self.params.insert(
            "sink_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sink_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guidevismode(mut self, val: DopFluidobjectSinkGuidevismode) -> Self {
        self.params.insert(
            "sink_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sink_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guidevisdensitymode(
        mut self,
        val: DopFluidobjectSinkGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "sink_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sink_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideplane(mut self, val: DopFluidobjectPumpGuideplane) -> Self {
        self.params.insert(
            "pump_guideplane".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pump_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guidevismode(mut self, val: DopFluidobjectPumpGuidevismode) -> Self {
        self.params.insert(
            "pump_guidevismode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pump_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guidevismode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guidevisdensitymode(
        mut self,
        val: DopFluidobjectPumpGuidevisdensitymode,
    ) -> Self {
        self.params.insert(
            "pump_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pump_guidevisdensitymode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guidevisdensitymode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_border(mut self, val: DopFluidobjectBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_object_name(mut self, val: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_object_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_points(mut self, val: &str) -> Self {
        self.params.insert(
            "instance_points".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_instance_points_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_points".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "surface_soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_surface_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "velocity_soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_velocity_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_positionpath(mut self, val: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_positionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_twod(mut self, val: bool) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allowcaching(mut self, val: bool) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowcaching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowcaching".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_instance_objects(mut self, val: bool) -> Self {
        self.params.insert(
            "instance_objects".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_instance_objects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "instance_objects".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_override_container_size(mut self, val: bool) -> Self {
        self.params.insert(
            "override_container_size".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_override_container_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "override_container_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_override_division_size(mut self, val: bool) -> Self {
        self.params.insert(
            "override_division_size".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_override_division_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "override_division_size".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_continuous(mut self, val: bool) -> Self {
        self.params.insert(
            "continuous".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_continuous_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "continuous".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_showguide".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_showguide".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_show_surfels_display(mut self, val: bool) -> Self {
        self.params.insert(
            "show_surfels_display".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_show_surfels_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "show_surfels_display".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surface_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_surface_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surface_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideusebox(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideusebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideusebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideusebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideuseboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideuseboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideuseboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideusesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideusesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideusesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideoverridediv(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideoverridediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideoverridediv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidebarbs(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guidebarbs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guidebarbs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidebarbs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guidepercomp(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guidepercomp".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guidepercomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guidepercomp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideusestreamers(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideusestreamers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideusestreamers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velocity_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "velocity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_velocity_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_source_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_source_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sink_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "sink_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sink_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sink_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_usebox".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_usebox".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_useboxhash".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_useboxhash".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_usesmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_usesmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_usesmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_usesmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideremapsmoke(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guideremapsmoke_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideremapsmoke".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guideuseplane".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideuseplane".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guiderangemode(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guiderangemode".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guiderangemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guiderangemode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pump_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "pump_guideinvert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_pump_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pump_guideinvert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closedends(mut self, val: bool) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closedends_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closedends".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closexpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closexpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closexpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeyneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeyneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeyneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closeypos(mut self, val: bool) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closeypos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closeypos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezneg(mut self, val: bool) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezneg".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_closezpos(mut self, val: bool) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_closezpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "closezpos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFluidobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fluidobject"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
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
pub enum DopFluidsolverSourcevelmerge {
    None = 0,
    NetNewSource = 1,
    NewSource = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverSourceveltype {
    RigidVelocity = 0,
    PointVelocity = 1,
    VolumeVelocity = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverPumpvelmerge {
    Blend = 0,
    Add = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverPumpveltype {
    RigidVelocity = 0,
    PointVelocity = 1,
    VolumeVelocity = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverCollisionveltype {
    RigidVelocity = 0,
    PointVelocity = 1,
    VolumeVelocity = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverAdvecttype {
    SingleStage = 0,
    Bfecc = 1,
    ModifiedMaccormack = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverAdvectclampvalues {
    None = 0,
    Clamp = 1,
    Revert = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverVeladvecttype {
    SingleStage = 0,
    Bfecc = 1,
    ModifiedMaccormack = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverVeladvectclampvalues {
    None = 0,
    Clamp = 1,
    Revert = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverAdvectmethod {
    SingleStep = 0,
    Trace = 1,
    TraceMidpoint = 2,
    Hjweno = 3,
    Upwind = 4,
    TraceRk3 = 5,
    TraceRk4 = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverCollisionweightSampling {
    Supersampling = 0,
    VoxelFaceArea = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopFluidsolverFieldsclear {
    None = 0,
    Hidden = 1,
    Static = 2,
}

#[derive(Debug, Clone)]
pub struct DopFluidsolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl DopFluidsolver {
    pub const OUT_OUTPUT1: &'static str = "output1";

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
        self.inputs.insert(
            index,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_at_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        index: usize,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            index,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 0: "Object"
    pub fn set_input_object<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            0,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 0: "Object" and specifies the output index of the target node.
    pub fn set_input_object_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_object_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            0,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 1: "Pre-solve"
    pub fn set_input_pre_solve<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            1,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 1: "Pre-solve" and specifies the output index of the target node.
    pub fn set_input_pre_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_pre_solve_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            1,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 2: "Velocity Update"
    pub fn set_input_velocity_update<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            2,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 2: "Velocity Update" and specifies the output index of the target node.
    pub fn set_input_velocity_update_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_velocity_update_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            2,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 3: "Advection"
    pub fn set_input_advection<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(
            3,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 3: "Advection" and specifies the output index of the target node.
    pub fn set_input_advection_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_advection_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            3,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    /// Connects to input 4: "Sourcing (post-solve)"
    pub fn set_input_sourcing_post_solve<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(
            4,
            (target.get_id(), crate::core::types::OutputPort::Index(0)),
        );
        self
    }

    /// Connects to input 4: "Sourcing (post-solve)" and specifies the output index of the target node.
    pub fn set_input_sourcing_post_solve_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Index(output_index),
            ),
        );
        self
    }

    pub fn set_input_sourcing_post_solve_by_name<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        port_name: &str,
    ) -> Self {
        self.inputs.insert(
            4,
            (
                target.get_id(),
                crate::core::types::OutputPort::Name(port_name.to_string()),
            ),
        );
        self
    }

    // --- Float parameters ---
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_confinement(mut self, val: f32) -> Self {
        self.params.insert(
            "confinement".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_confinement_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "confinement".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_speedlimit(mut self, val: f32) -> Self {
        self.params.insert(
            "speedlimit".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_speedlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speedlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_viscosity(mut self, val: f32) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_viscosity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "viscosity".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pumpvelscale(mut self, val: f32) -> Self {
        self.params.insert(
            "pumpvelscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pumpvelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pumpvelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cflcond(mut self, val: f32) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cflcond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reinitializerate(mut self, val: f32) -> Self {
        self.params.insert(
            "reinitializerate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_reinitializerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reinitializerate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_escapethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "escapethreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_escapethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "escapethreshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfels_minrad(mut self, val: f32) -> Self {
        self.params.insert(
            "surfels_minrad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfels_minrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfels_minrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfels_maxrad(mut self, val: f32) -> Self {
        self.params.insert(
            "surfels_maxrad".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_surfels_maxrad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfels_maxrad".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_velextrapolatemaxcells(mut self, val: f32) -> Self {
        self.params.insert(
            "velextrapolatemaxcells".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_velextrapolatemaxcells_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velextrapolatemaxcells".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strain_alpha(mut self, val: f32) -> Self {
        self.params.insert(
            "strain_alpha".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strain_alpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strain_alpha".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strain_gamma(mut self, val: f32) -> Self {
        self.params.insert(
            "strain_gamma".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strain_gamma_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strain_gamma".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_strain_elasticmodulus(mut self, val: f32) -> Self {
        self.params.insert(
            "strain_elasticmodulus".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strain_elasticmodulus_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strain_elasticmodulus".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advect_cfl(mut self, val: f32) -> Self {
        self.params.insert(
            "advect_cfl".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_advect_cfl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "advect_cfl".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_feedbackscale(mut self, val: f32) -> Self {
        self.params.insert(
            "feedbackscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_feedbackscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "feedbackscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_minimumsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_frames_before_solve(mut self, val: i32) -> Self {
        self.params.insert(
            "frames_before_solve".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_frames_before_solve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frames_before_solve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reinitializeiter(mut self, val: i32) -> Self {
        self.params.insert(
            "reinitializeiter".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_reinitializeiter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reinitializeiter".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_surfelpervoxel(mut self, val: i32) -> Self {
        self.params.insert(
            "surfelpervoxel".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_surfelpervoxel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfelpervoxel".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionweight_numsupersamples(mut self, val: i32) -> Self {
        self.params.insert(
            "collisionweight_numsupersamples".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_collisionweight_numsupersamples_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionweight_numsupersamples".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_port(mut self, val: i32) -> Self {
        self.params
            .insert("port".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_port_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "port".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slice(mut self, val: i32) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_numslice(mut self, val: i32) -> Self {
        self.params.insert(
            "numslice".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_sourcevelmerge(mut self, val: DopFluidsolverSourcevelmerge) -> Self {
        self.params.insert(
            "sourcevelmerge".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourcevelmerge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcevelmerge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_sourceveltype(mut self, val: DopFluidsolverSourceveltype) -> Self {
        self.params.insert(
            "sourceveltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sourceveltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourceveltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pumpvelmerge(mut self, val: DopFluidsolverPumpvelmerge) -> Self {
        self.params.insert(
            "pumpvelmerge".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pumpvelmerge_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pumpvelmerge".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pumpveltype(mut self, val: DopFluidsolverPumpveltype) -> Self {
        self.params.insert(
            "pumpveltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pumpveltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pumpveltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionveltype(mut self, val: DopFluidsolverCollisionveltype) -> Self {
        self.params.insert(
            "collisionveltype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionveltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionveltype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advecttype(mut self, val: DopFluidsolverAdvecttype) -> Self {
        self.params.insert(
            "advecttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_advecttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "advecttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advectclampvalues(mut self, val: DopFluidsolverAdvectclampvalues) -> Self {
        self.params.insert(
            "advectclampvalues".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_advectclampvalues_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "advectclampvalues".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veladvecttype(mut self, val: DopFluidsolverVeladvecttype) -> Self {
        self.params.insert(
            "veladvecttype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_veladvecttype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veladvecttype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_veladvectclampvalues(mut self, val: DopFluidsolverVeladvectclampvalues) -> Self {
        self.params.insert(
            "veladvectclampvalues".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_veladvectclampvalues_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "veladvectclampvalues".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_advectmethod(mut self, val: DopFluidsolverAdvectmethod) -> Self {
        self.params.insert(
            "advectmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_advectmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "advectmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collisionweight_sampling(
        mut self,
        val: DopFluidsolverCollisionweightSampling,
    ) -> Self {
        self.params.insert(
            "collisionweight_sampling".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_collisionweight_sampling_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisionweight_sampling".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fieldsclear(mut self, val: DopFluidsolverFieldsclear) -> Self {
        self.params.insert(
            "fieldsclear".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fieldsclear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fieldsclear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_extraclear(mut self, val: &str) -> Self {
        self.params.insert(
            "extraclear".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_extraclear_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraclear".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_address(mut self, val: &str) -> Self {
        self.params.insert(
            "address".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_address_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "address".to_string(),
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
    pub fn with_group(mut self, val: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dataname(mut self, val: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_dospeedlimit(mut self, val: bool) -> Self {
        self.params.insert(
            "dospeedlimit".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dospeedlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dospeedlimit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usephysparms(mut self, val: bool) -> Self {
        self.params.insert(
            "usephysparms".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usephysparms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usephysparms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_relationships(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_relationships".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_relationships_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_relationships".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_source_relationship(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_source_relationship".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_source_relationship_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_source_relationship".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_pump_relationship(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_pump_relationship".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_pump_relationship_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_pump_relationship".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_collision_relationship(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_collision_relationship".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_collision_relationship_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_collision_relationship".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_collision_usebandwidth(mut self, val: bool) -> Self {
        self.params.insert(
            "collision_usebandwidth".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collision_usebandwidth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collision_usebandwidth".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_stickyfluid(mut self, val: bool) -> Self {
        self.params.insert(
            "stickyfluid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickyfluid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyfluid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_allownonsdf(mut self, val: bool) -> Self {
        self.params.insert(
            "allownonsdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allownonsdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allownonsdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_sink_relationship(mut self, val: bool) -> Self {
        self.params.insert(
            "enable_sink_relationship".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_sink_relationship_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable_sink_relationship".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quantize(mut self, val: bool) -> Self {
        self.params.insert(
            "quantize".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_quantize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantize".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_correctsurface(mut self, val: bool) -> Self {
        self.params.insert(
            "correctsurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_correctsurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "correctsurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rebuildsdf(mut self, val: bool) -> Self {
        self.params.insert(
            "rebuildsdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_rebuildsdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rebuildsdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_reinitializesdf(mut self, val: bool) -> Self {
        self.params.insert(
            "reinitializesdf".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reinitializesdf_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reinitializesdf".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_insidemarkers(mut self, val: bool) -> Self {
        self.params.insert(
            "insidemarkers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_insidemarkers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "insidemarkers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_outsidemarkers(mut self, val: bool) -> Self {
        self.params.insert(
            "outsidemarkers".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_outsidemarkers_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outsidemarkers".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variational(mut self, val: bool) -> Self {
        self.params.insert(
            "variational".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variational_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "variational".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scalebysurface(mut self, val: bool) -> Self {
        self.params.insert(
            "scalebysurface".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_scalebysurface_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scalebysurface".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preservebubble(mut self, val: bool) -> Self {
        self.params.insert(
            "preservebubble".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preservebubble_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preservebubble".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ghostfluid(mut self, val: bool) -> Self {
        self.params.insert(
            "ghostfluid".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ghostfluid_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ghostfluid".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitvelextrapolation(mut self, val: bool) -> Self {
        self.params.insert(
            "limitvelextrapolation".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitvelextrapolation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitvelextrapolation".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_usestrain(mut self, val: bool) -> Self {
        self.params.insert(
            "usestrain".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usestrain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usestrain".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dodistributedsolve(mut self, val: bool) -> Self {
        self.params.insert(
            "dodistributedsolve".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dodistributedsolve_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dodistributedsolve".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uniquedataname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquedataname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquedataname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for DopFluidsolver {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fluidsolver"
    }

    fn get_inputs(
        &self,
    ) -> &std::collections::BTreeMap<usize, (usize, crate::core::types::OutputPort)> {
        &self.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.spare_params
    }
}
