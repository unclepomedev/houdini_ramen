#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLagLagmethod {
    LagValue = 0,
    LagAmplitude = 1,
    LagMagnitude = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLagSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLagUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopLag {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopLag {
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

    /// Connects to input 0: "Lag Source"
    pub fn set_input_lag_source<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Lag Source" and specifies the output index of the target node.
    pub fn set_input_lag_source_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_lag(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "lag".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_lag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_overshoot(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "overshoot".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_overshoot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overshoot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_slope(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "slope".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_slope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_accel(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "accel".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_accel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_lagmethod(mut self, val: ChopLagLagmethod) -> Self {
        self.params.insert(
            "lagmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lagmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lagmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopLagSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopLagUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_clamp(mut self, val: bool) -> Self {
        self.params.insert(
            "clamp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_clamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_aclamp(mut self, val: bool) -> Self {
        self.params.insert(
            "aclamp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_aclamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aclamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopLag {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lag"
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
pub enum ChopLayerSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLayerUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopLayer {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl ChopLayer {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Button parameters ---
    pub fn trigger_updatelayernames(mut self) -> Self {
        self.params.insert(
            "updatelayernames".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_weight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("weight{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("weight{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_active(mut self, val: i32) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_active_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_srselect(mut self, val: ChopLayerSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopLayerUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("name{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_mute_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("mute{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mute_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("mute{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_solo_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("solo{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solo_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("solo{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopLayer {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "layer"
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
pub enum ChopLimitType {
    Off = 0,
    Clamp = 1,
    Loop = 2,
    Zigzag = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLimitQuantvalue {
    Off = 0,
    Ceiling = 1,
    Floor = 2,
    Round = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLimitQuantindex {
    Off = 0,
    OffsetRelativeToStart = 1,
    OffsetRelativeToZero = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLimitSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLimitUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopLimit {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopLimit {
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

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_min(mut self, val: f32) -> Self {
        self.params.insert(
            "min".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_min_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "min".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_max(mut self, val: f32) -> Self {
        self.params.insert(
            "max".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vstep(mut self, val: f32) -> Self {
        self.params.insert(
            "vstep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_voffset(mut self, val: f32) -> Self {
        self.params.insert(
            "voffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_voffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_istep(mut self, val: f32) -> Self {
        self.params.insert(
            "istep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_istep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "istep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ioffset(mut self, val: f32) -> Self {
        self.params.insert(
            "ioffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ioffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ioffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: ChopLimitType) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "type".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quantvalue(mut self, val: ChopLimitQuantvalue) -> Self {
        self.params.insert(
            "quantvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quantvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_quantindex(mut self, val: ChopLimitQuantindex) -> Self {
        self.params.insert(
            "quantindex".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_quantindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "quantindex".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopLimitSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopLimitUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_positive(mut self, val: bool) -> Self {
        self.params.insert(
            "positive".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_positive_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positive".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_norm(mut self, val: bool) -> Self {
        self.params.insert(
            "norm".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_norm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "norm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopLimit {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "limit"
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
pub enum ChopLogicConvert {
    OffWhenZero = 0,
    OffWhenZeroOrLess = 1,
    OffWhenOutsideBounds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicPreop {
    Off = 0,
    Invert = 1,
    Toggle = 2,
    RadioButton = 3,
    LastTwoOn = 4,
    RisingEdge = 5,
    FallingEdge = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicChanop {
    Off = 0,
    And = 1,
    Or = 2,
    ExclusiveOr = 3,
    NotAnd = 4,
    NotOr = 5,
    Equivalence = 6,
    FirstChannelOn = 7,
    LastChannelOn = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicChopop {
    Off = 0,
    And = 1,
    Or = 2,
    ExclusiveOr = 3,
    NotAnd = 4,
    NotOr = 5,
    Equivalence = 6,
    FirstChannelOn = 7,
    LastChannelOn = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicMatch {
    ChannelNumber = 0,
    ChannelName = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicMatchfailure {
    Error = 0,
    Warning = 1,
    Ignore = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicAlign {
    /// Extend to Min/Max
    ExtendToMinMax = 0,
    /// Stretch to Min/Max
    StretchToMinMax = 1,
    ShiftToMinimum = 2,
    ShiftToMaximum = 3,
    ShiftToFirstInterval = 4,
    TrimToFirstInterval = 5,
    StretchToFirstInterval = 6,
    TrimToSmallestInterval = 7,
    StretchToSmallestInterval = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLogicUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopLogic {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl ChopLogic {
    pub fn new(name: &str) -> Self {
        Self {
            id: houdini_ramen_core::types::NODE_ID_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: name.to_string(),
            inputs: std::collections::BTreeMap::new(),
            params: std::collections::HashMap::new(),
            spare_params: Vec::new(),
            next_input_index: 0,
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

    /// Adds an input automatically to the next available index.
    pub fn add_input<N: houdini_ramen_core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), 0));
        self.next_input_index += 1;
        self
    }

    /// Adds an input automatically to the next available index and specifies the output index of the target node.
    pub fn add_input_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs
            .insert(self.next_input_index, (target.get_id(), output_index));
        self.next_input_index += 1;
        self
    }

    // --- Float parameters ---
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_bound(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "bound".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_bound_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bound".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_convert(mut self, val: ChopLogicConvert) -> Self {
        self.params.insert(
            "convert".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_convert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preop(mut self, val: ChopLogicPreop) -> Self {
        self.params.insert(
            "preop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_chanop(mut self, val: ChopLogicChanop) -> Self {
        self.params.insert(
            "chanop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_chanop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chanop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_chopop(mut self, val: ChopLogicChopop) -> Self {
        self.params.insert(
            "chopop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_chopop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chopop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_match(mut self, val: ChopLogicMatch) -> Self {
        self.params.insert(
            "match".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_match_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "match".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchfailure(mut self, val: ChopLogicMatchfailure) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchfailure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_align(mut self, val: ChopLogicAlign) -> Self {
        self.params.insert(
            "align".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_align_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "align".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopLogicSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopLogicUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scriptoffon(mut self, val: &str) -> Self {
        self.params.insert(
            "scriptoffon".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scriptoffon_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scriptoffon".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scripton(mut self, val: &str) -> Self {
        self.params.insert(
            "scripton".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scripton_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scripton".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scriptonoff(mut self, val: &str) -> Self {
        self.params.insert(
            "scriptonoff".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scriptonoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scriptonoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scriptoff(mut self, val: &str) -> Self {
        self.params.insert(
            "scriptoff".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scriptoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scriptoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopLogic {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "logic"
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
pub enum ChopLookupChanmatch {
    OneLookupTableChannel = 0,
    AllLookupTableChannels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLookupMatch {
    ChannelNumber = 0,
    ChannelName = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLookupMatchfailure {
    Error = 0,
    Warning = 1,
    Ignore = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLookupSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopLookupUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopLookup {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopLookup {
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

    /// Connects to input 0: "Index"
    pub fn set_input_index<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Index" and specifies the output index of the target node.
    pub fn set_input_index_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Lookup Table"
    pub fn set_input_lookup_table<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Lookup Table" and specifies the output index of the target node.
    pub fn set_input_lookup_table_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_index(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_index_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "index".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_chanmatch(mut self, val: ChopLookupChanmatch) -> Self {
        self.params.insert(
            "chanmatch".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_chanmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chanmatch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_match(mut self, val: ChopLookupMatch) -> Self {
        self.params.insert(
            "match".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_match_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "match".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchfailure(mut self, val: ChopLookupMatchfailure) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchfailure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopLookupSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopLookupUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopLookup {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lookup"
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

#[derive(Debug, Clone)]
pub struct ChopLopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopLopnet {
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

    // --- Int parameters ---
    pub fn with_modifiedprimcounttostartnewlayer(mut self, val: i32) -> Self {
        self.params.insert(
            "modifiedprimcounttostartnewlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_modifiedprimcounttostartnewlayer_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "modifiedprimcounttostartnewlayer".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_expansioneffect(mut self, val: &str) -> Self {
        self.params.insert(
            "expansioneffect".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_expansioneffect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expansioneffect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinnedprims(mut self, val: &str) -> Self {
        self.params.insert(
            "pinnedprims".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pinnedprims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinnedprims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextassetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextassetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvercontextassetpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringurlprefix_inst_expr(
        mut self,
        index1: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("resolvercontextstringurlprefix{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_resolvercontextstringvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionset_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantselectionset_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionset{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionvalue_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_variantselectionvalue_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionvalue{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_insertionpointdescriptor(mut self, val: &str) -> Self {
        self.params.insert(
            "insertionpointdescriptor".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_insertionpointdescriptor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "insertionpointdescriptor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rendergallerysource(mut self, val: &str) -> Self {
        self.params.insert(
            "rendergallerysource".to_string(),
            houdini_ramen_core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rendergallerysource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendergallerysource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_resolvercontextstringenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_resolvercontextstringenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("resolvercontextstringenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_variantselectionenable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_variantselectionenable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("variantselectionenable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopLopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "lopnet"
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
