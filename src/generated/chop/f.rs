#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFanFanop {
    FanOut = 0,
    FanIn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFanRange {
    ClampIndex = 0,
    LoopIndex = 1,
    SetChannelsTo0 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFanAlloff {
    SetTo0 = 0,
    /// Set To -1
    SetToMinus1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFanSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFanUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFan {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFan {
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
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_fanop(mut self, val: ChopFanFanop) -> Self {
        self.params.insert(
            "fanop".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fanop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fanop".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_range(mut self, val: ChopFanRange) -> Self {
        self.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_alloff(mut self, val: ChopFanAlloff) -> Self {
        self.params.insert(
            "alloff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_alloff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alloff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopFanSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFanUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_channame(mut self, val: &str) -> Self {
        self.params.insert(
            "channame".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_channame_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channame".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFan {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fan"
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
pub enum ChopFbxRateoption {
    NoChange = 0,
    Override = 1,
    Resample = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFbxLeft {
    NoChange = 0,
    Hold = 1,
    Slope = 2,
    Cycle = 3,
    Mirror = 4,
    DefaultValue = 5,
    CycleWithOffset = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFbxRight {
    NoChange = 0,
    Hold = 1,
    Slope = 2,
    Cycle = 3,
    Mirror = 4,
    DefaultValue = 5,
    CycleWithOffset = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFbxSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFbxUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFbx {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFbx {
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

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_fbxsamplerate(mut self, val: f32) -> Self {
        self.params.insert(
            "fbxsamplerate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fbxsamplerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fbxsamplerate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rate(mut self, val: f32) -> Self {
        self.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defval(mut self, val: f32) -> Self {
        self.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_rateoption(mut self, val: ChopFbxRateoption) -> Self {
        self.params.insert(
            "rateoption".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rateoption_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rateoption".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left(mut self, val: ChopFbxLeft) -> Self {
        self.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right(mut self, val: ChopFbxRight) -> Self {
        self.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopFbxSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFbxUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
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
    pub fn with_fbxtake(mut self, val: &str) -> Self {
        self.params.insert(
            "fbxtake".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_fbxtake_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fbxtake".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opmenu(mut self, val: &str) -> Self {
        self.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_fbxoverridesamplerate(mut self, val: bool) -> Self {
        self.params.insert(
            "fbxoverridesamplerate".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fbxoverridesamplerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fbxoverridesamplerate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_convertunits(mut self, val: bool) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_convertunits_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "convertunits".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_removenamespaces(mut self, val: bool) -> Self {
        self.params.insert(
            "removenamespaces".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_removenamespaces_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "removenamespaces".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_minimalnodes(mut self, val: bool) -> Self {
        self.params.insert(
            "minimalnodes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_minimalnodes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimalnodes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keepdeformingshapexforms(mut self, val: bool) -> Self {
        self.params.insert(
            "keepdeformingshapexforms".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keepdeformingshapexforms_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keepdeformingshapexforms".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFbx {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fbx"
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
pub trait ChopFbxInnerExt {
    fn file1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ChopFbxInnerExt for crate::core::graph::InnerGraph<'a> {
    fn file1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("file1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFeedbackOutput {
    PreviousChannelsAtPreviousTime = 0,
    PreviousChannelsAtCurrentTime = 1,
    LastSampleAtCurrentTime = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFeedbackSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFeedbackUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFeedback {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFeedback {
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

    /// Connects to input 0: "Feedback Loop"
    pub fn set_input_feedback_loop<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Feedback Loop" and specifies the output index of the target node.
    pub fn set_input_feedback_loop_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Initial Condition"
    pub fn set_input_initial_condition<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Initial Condition" and specifies the output index of the target node.
    pub fn set_input_initial_condition_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Reset Pulse"
    pub fn set_input_reset_pulse<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Reset Pulse" and specifies the output index of the target node.
    pub fn set_input_reset_pulse_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_output(mut self, val: ChopFeedbackOutput) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopFeedbackSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFeedbackUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_delta(mut self, val: bool) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_delta_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delta".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFeedback {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "feedback"
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
pub enum ChopFetchData {
    OpControlChannels = 0,
    ChopDataChannels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFetchSelect {
    AllChannels = 0,
    CreatedChannels = 1,
    /// Non-Constant Channels
    NonMinusConstantChannels = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFetchNameformat {
    ChannelName = 0,
    OpAndChannelNames = 1,
    FullPathName = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFetchRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFetchSample {
    /// New Rate (Same Index Range)
    NewRateSameIndexRange = 0,
    ResampleFromMaximumRate = 1,
    ResampleFromMinimumRate = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFetchSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFetchUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFetch {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFetch {
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
    pub fn with_start(mut self, val: f32) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_end(mut self, val: f32) -> Self {
        self.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rate(mut self, val: f32) -> Self {
        self.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_data(mut self, val: ChopFetchData) -> Self {
        self.params.insert(
            "data".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_data_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "data".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_select(mut self, val: ChopFetchSelect) -> Self {
        self.params.insert(
            "select".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_select_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "select".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_nameformat(mut self, val: ChopFetchNameformat) -> Self {
        self.params.insert(
            "nameformat".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_nameformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameformat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_range(mut self, val: ChopFetchRange) -> Self {
        self.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
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
    pub fn with_sample(mut self, val: ChopFetchSample) -> Self {
        self.params.insert(
            "sample".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_sample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopFetchSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFetchUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_nodepath(mut self, val: &str) -> Self {
        self.params.insert(
            "nodepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nodepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nodepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_path(mut self, val: &str) -> Self {
        self.params.insert(
            "path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFetch {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fetch"
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
pub enum ChopFetchchopSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFetchchopUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFetchchop {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFetchchop {
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
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_srselect(mut self, val: ChopFetchchopSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFetchchopUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_nodepath(mut self, val: &str) -> Self {
        self.params.insert(
            "nodepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_nodepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nodepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFetchchop {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "fetchchop"
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
pub enum ChopFileNameoption {
    UseNamesInFile = 0,
    UseNewNames = 1,
    UseFilename = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFileRateoption {
    NoChange = 0,
    Override = 1,
    Resample = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFileLeft {
    NoChange = 0,
    Hold = 1,
    Slope = 2,
    Cycle = 3,
    Mirror = 4,
    DefaultValue = 5,
    CycleWithOffset = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFileRight {
    NoChange = 0,
    Hold = 1,
    Slope = 2,
    Cycle = 3,
    Mirror = 4,
    DefaultValue = 5,
    CycleWithOffset = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFileSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFileUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFile {
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

    // --- Button parameters ---
    pub fn trigger_reload(mut self) -> Self {
        self.params
            .insert("reload".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_rate(mut self, val: f32) -> Self {
        self.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defval(mut self, val: f32) -> Self {
        self.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_nameoption(mut self, val: ChopFileNameoption) -> Self {
        self.params.insert(
            "nameoption".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_nameoption_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameoption".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rateoption(mut self, val: ChopFileRateoption) -> Self {
        self.params.insert(
            "rateoption".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rateoption_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rateoption".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left(mut self, val: ChopFileLeft) -> Self {
        self.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right(mut self, val: ChopFileRight) -> Self {
        self.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopFileSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFileUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
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
    pub fn with_name(mut self, val: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFile {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "file"
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
pub enum ChopFilterType {
    Gaussian = 0,
    LeftHalfGaussian = 1,
    Box = 2,
    LeftHalfBox = 3,
    EdgeDetect = 4,
    Sharpen = 5,
    /// De-spike
    DeMinusSpike = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFilterSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFilterUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFilter {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFilter {
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
    pub fn with_effect(mut self, val: f32) -> Self {
        self.params.insert(
            "effect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "effect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_width(mut self, val: f32) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_width_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "width".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_spike(mut self, val: f32) -> Self {
        self.params.insert(
            "spike".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spike_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spike".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_passes(mut self, val: i32) -> Self {
        self.params.insert(
            "passes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_passes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "passes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type(mut self, val: ChopFilterType) -> Self {
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
    pub fn with_srselect(mut self, val: ChopFilterSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFilterUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFilter {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "filter"
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
pub enum ChopFootplantSource {
    InputChannels = 0,
    Object = 1,
    Agent = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFootplantMethod {
    SpeedThreshold = 0,
    DistanceFromTerrain = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFootplantTerrainsource {
    Sop = 0,
    Plane = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFootplantSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFootplantUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFootplant {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFootplant {
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

    /// Connects to input 0: "Translation Channels"
    pub fn set_input_translation_channels<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Translation Channels" and specifies the output index of the target node.
    pub fn set_input_translation_channels_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_speedthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "speedthreshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_speedthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "speedthreshold".to_string(),
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
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_framerange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "framerange".to_string(),
            crate::core::types::ParamValue::Float2(val),
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

    // --- Float3 parameters ---
    pub fn with_offset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planeorigin(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "planeorigin".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_planeorigin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planeorigin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_planenormal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "planenormal".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_planenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "planenormal".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_blendinframes(mut self, val: i32) -> Self {
        self.params.insert(
            "blendinframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blendinframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blendinframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendoutframes(mut self, val: i32) -> Self {
        self.params.insert(
            "blendoutframes".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_blendoutframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blendoutframes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_source(mut self, val: ChopFootplantSource) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: ChopFootplantMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrainsource(mut self, val: ChopFootplantTerrainsource) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_terrainsource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrainsource".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopFootplantSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFootplantUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
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
    pub fn with_clipname(mut self, val: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_clipname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_transformname(mut self, val: &str) -> Self {
        self.params.insert(
            "transformname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_transformname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "transformname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_targetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_referencepath(mut self, val: &str) -> Self {
        self.params.insert(
            "referencepath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_referencepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "referencepath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_terrain_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "terrain_soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_terrain_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "terrain_soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channelname(mut self, val: &str) -> Self {
        self.params.insert(
            "channelname".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_channelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channelname".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opmenu(mut self, val: &str) -> Self {
        self.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opmenu_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFootplant {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "footplant"
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
pub trait ChopFootplantInnerExt {
    fn channels_for_sop(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn add_loop(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn agent_clip(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn apply_locomotion(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn apply_transform(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn apply_transform1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn blend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn compute_distance(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn fetch_signed_distance(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn fetch_world_transform(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn find_stationary_regions(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input_positions(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn isolate_joint(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn locomotion(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn offset(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn rename(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn reset_extend(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn retrim(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn switch_method(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn to_speed(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn to_velocity(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn trim1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn trim_last_frame(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ChopFootplantInnerExt for crate::core::graph::InnerGraph<'a> {
    fn channels_for_sop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("CHANNELS_FOR_SOP")
    }
    fn out(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("OUT")
    }
    fn add_loop(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("add_loop")
    }
    fn agent_clip(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("agent_clip")
    }
    fn apply_locomotion(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("apply_locomotion")
    }
    fn apply_transform(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("apply_transform")
    }
    fn apply_transform1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("apply_transform1")
    }
    fn blend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("blend")
    }
    fn compute_distance(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("compute_distance")
    }
    fn fetch_signed_distance(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("fetch_signed_distance")
    }
    fn fetch_world_transform(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("fetch_world_transform")
    }
    fn find_stationary_regions(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("find_stationary_regions")
    }
    fn input_positions(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("input_positions")
    }
    fn isolate_joint(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("isolate_joint")
    }
    fn locomotion(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("locomotion")
    }
    fn offset(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("offset")
    }
    fn rename(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("rename")
    }
    fn reset_extend(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("reset_extend")
    }
    fn retrim(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("retrim")
    }
    fn switch_method(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("switch_method")
    }
    fn to_speed(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("to_speed")
    }
    fn to_velocity(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("to_velocity")
    }
    fn trim1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("trim1")
    }
    fn trim_last_frame(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("trim_last_frame")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopForSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopForUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFor {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFor {
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
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_loopindex(mut self, val: i32) -> Self {
        self.params.insert(
            "loopindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_loopindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loopindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_srselect(mut self, val: ChopForSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopForUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_loopgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "loopgroup".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_loopgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loopgroup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFor {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "for"
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
pub enum ChopForeachIterate {
    OverChannelGroups = 0,
    OverNumbers = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopForeachChanindex {
    Append = 0,
    ReplaceToken = 1,
    InsertAtFirstSeparator = 2,
    InsertAtLastSeparator = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopForeachSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopForeachUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopForeach {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopForeach {
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

    /// Connects to input 0: "Channels to Iterate Over"
    pub fn set_input_channels_to_iterate_over<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Channels to Iterate Over" and specifies the output index of the target node.
    pub fn set_input_channels_to_iterate_over_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Constant Input 2"
    pub fn set_input_constant_input_2<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Constant Input 2" and specifies the output index of the target node.
    pub fn set_input_constant_input_2_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Constant Input 3"
    pub fn set_input_constant_input_3<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Constant Input 3" and specifies the output index of the target node.
    pub fn set_input_constant_input_3_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Constant Input 4"
    pub fn set_input_constant_input_4<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Constant Input 4" and specifies the output index of the target node.
    pub fn set_input_constant_input_4_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_count(mut self, val: i32) -> Self {
        self.params.insert(
            "count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_baseindex(mut self, val: i32) -> Self {
        self.params.insert(
            "baseindex".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_baseindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_iterate(mut self, val: ChopForeachIterate) -> Self {
        self.params.insert(
            "iterate".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_iterate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "iterate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_chanindex(mut self, val: ChopForeachChanindex) -> Self {
        self.params.insert(
            "chanindex".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_chanindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chanindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopForeachSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopForeachUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_changrp(mut self, val: &str) -> Self {
        self.params.insert(
            "changrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_changrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "changrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_chanreplace(mut self, val: &str) -> Self {
        self.params.insert(
            "chanreplace".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_chanreplace_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chanreplace".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_previewgrp(mut self, val: &str) -> Self {
        self.params.insert(
            "previewgrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_previewgrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "previewgrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debuggrp(mut self, val: &str) -> Self {
        self.params.insert(
            "debuggrp".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_debuggrp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "debuggrp".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_reindex(mut self, val: bool) -> Self {
        self.params.insert(
            "reindex".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_reindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "reindex".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_preview(mut self, val: bool) -> Self {
        self.params.insert(
            "preview".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_preview_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preview".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_debug(mut self, val: bool) -> Self {
        self.params.insert(
            "debug".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_debug_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "debug".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopForeach {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "foreach"
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
pub trait ChopForeachInnerExt {
    fn foreach_iteration(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention)]
impl<'a> ChopForeachInnerExt for crate::core::graph::InnerGraph<'a> {
    fn foreach_iteration(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Foreach_Iteration")
    }
    fn output(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.get_existing_node("Output")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFunctionFunc {
    /// sqrt(x) Square Root
    SqrtXSquareRoot = 0,
    /// abs(x) Absolute Value
    AbsXAbsoluteValue = 1,
    /// sign(x) Sign
    SignXSign = 2,
    /// cos(x) Cosine
    CosXCosine = 3,
    /// sin(x) Sine
    SinXSine = 4,
    /// tan(x) Tangent
    TanXTangent = 5,
    /// acos(x) Arccosine
    AcosXArccosine = 6,
    /// asin(x) Arcsine
    AsinXArcsine = 7,
    /// atan(x) Arctan( Input1 )
    AtanXArctanInput1 = 8,
    /// atan2(x,y) Arctan( Input1 / Input2 )
    Atan2XYArctanInput1Input2 = 9,
    /// cosh(x) Hyperbolic Cosine
    CoshXHyperbolicCosine = 10,
    /// sinh(x) Hyperbolic Sine
    SinhXHyperbolicSine = 11,
    /// tanh(x) Hyperbolic Tangent
    TanhXHyperbolicTangent = 12,
    /// log10(x) Log base 10
    Log10XLogBase10 = 13,
    /// logN(x) Log base N
    LognXLogBaseN = 14,
    /// ln(x) Natural Log
    LnXNaturalLog = 15,
    /// pow(10,x) 10 ^ Input1
    Pow10X10Input1 = 16,
    /// exp(x) e ^ Input1
    ExpXEInput1 = 17,
    /// pow(x) Base ^ Input1
    PowXBaseInput1 = 18,
    /// pow(x) Input1 ^ Exponent
    PowXInput1Exponent = 19,
    /// pow(x,y) Input1 ^ Input2
    PowXYInput1Input2 = 20,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFunctionAngunit {
    Degrees = 0,
    Radians = 1,
    Cycles = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFunctionMatch {
    ChannelNumber = 0,
    ChannelName = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFunctionMatchfailure {
    Error = 0,
    Warning = 1,
    Ignore = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFunctionError {
    AbortWithErrorMessage = 0,
    ReplaceWithSpecifiedValues = 1,
    UseThePreviousValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFunctionSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopFunctionUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopFunction {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopFunction {
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

    /// Connects to input 0: "Channels to Modify"
    pub fn set_input_channels_to_modify<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Channels to Modify" and specifies the output index of the target node.
    pub fn set_input_channels_to_modify_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Second Parameter"
    pub fn set_input_second_parameter<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Second Parameter" and specifies the output index of the target node.
    pub fn set_input_second_parameter_from<N: crate::core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_baseval(mut self, val: f32) -> Self {
        self.params.insert(
            "baseval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_baseval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "baseval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_expval(mut self, val: f32) -> Self {
        self.params.insert(
            "expval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_expval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "expval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pinfval(mut self, val: f32) -> Self {
        self.params.insert(
            "pinfval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pinfval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pinfval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ninfval(mut self, val: f32) -> Self {
        self.params.insert(
            "ninfval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ninfval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ninfval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_domval(mut self, val: f32) -> Self {
        self.params.insert(
            "domval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_domval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "domval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_divval(mut self, val: f32) -> Self {
        self.params.insert(
            "divval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_func(mut self, val: ChopFunctionFunc) -> Self {
        self.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_func_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "func".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_angunit(mut self, val: ChopFunctionAngunit) -> Self {
        self.params.insert(
            "angunit".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_angunit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angunit".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_match(mut self, val: ChopFunctionMatch) -> Self {
        self.params.insert(
            "match".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_match_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "match".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchfailure(mut self, val: ChopFunctionMatchfailure) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchfailure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_error(mut self, val: ChopFunctionError) -> Self {
        self.params.insert(
            "error".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_error_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "error".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopFunctionSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopFunctionUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopFunction {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "function"
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
