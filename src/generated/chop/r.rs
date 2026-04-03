#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordRecord {
    Off = 0,
    On = 1,
    Add = 2,
    AutoRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordSample {
    CurrentFrame = 0,
    CurrentTimeSlice = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordInterp {
    HoldPreviousValue = 0,
    Linear = 1,
    Cubic = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordType {
    Raw = 0,
    RawSpeed = 1,
    PositionOffset = 2,
    Speed = 3,
    /// Speed + Hold
    SpeedPlusHold = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordOutput {
    FullRange = 0,
    CurrentFrame = 1,
    CurrentFrameAtFrame1 = 2,
    CurrentTimeSlice = 3,
    Segment = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordSingleframetoggle {
    None = 0,
    Control = 1,
    Alt = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordInit {
    PositionOffset = 0,
    Speed = 1,
    SetAllValues = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordRead {
    PositionOffset = 0,
    Speed = 1,
    ReadAllValues = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordLimita {
    Off = 0,
    Clamp = 1,
    Loop = 2,
    Zigzag = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordLimitb {
    Off = 0,
    Clamp = 1,
    Loop = 2,
    Zigzag = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRecordUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopRecord {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopRecord {
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

    /// Connects to input 0: "Position"
    pub fn set_input_position<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Position" and specifies the output index of the target node.
    pub fn set_input_position_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Active"
    pub fn set_input_active<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Active" and specifies the output index of the target node.
    pub fn set_input_active_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Button parameters ---
    pub fn trigger_reset(mut self) -> Self {
        self.params.insert("reset".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_resync(mut self) -> Self {
        self.params.insert("resync".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_mina(mut self, val: f32) -> Self {
        self.params.insert("mina".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_mina_expr(mut self, expr: &str) -> Self {
        self.params.insert("mina".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxa(mut self, val: f32) -> Self {
        self.params.insert("maxa".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxa_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxa".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gaina(mut self, val: f32) -> Self {
        self.params.insert("gaina".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gaina_expr(mut self, expr: &str) -> Self {
        self.params.insert("gaina".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_minb(mut self, val: f32) -> Self {
        self.params.insert("minb".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_minb_expr(mut self, expr: &str) -> Self {
        self.params.insert("minb".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_maxb(mut self, val: f32) -> Self {
        self.params.insert("maxb".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_maxb_expr(mut self, expr: &str) -> Self {
        self.params.insert("maxb".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gainb(mut self, val: f32) -> Self {
        self.params.insert("gainb".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gainb_expr(mut self, expr: &str) -> Self {
        self.params.insert("gainb".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float2 parameters ---
    pub fn with_segment(mut self, val: [f32; 2]) -> Self {
        self.params.insert("segment".to_string(), crate::core::types::ParamValue::Float2(val));
        self
    }
    pub fn with_segment_expr(mut self, expr: &str) -> Self {
        self.params.insert("segment".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Int parameters ---
    pub fn with_chanperlim(mut self, val: i32) -> Self {
        self.params.insert("chanperlim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_chanperlim_expr(mut self, expr: &str) -> Self {
        self.params.insert("chanperlim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_record(mut self, val: ChopRecordRecord) -> Self {
        self.params.insert("record".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_record_expr(mut self, expr: &str) -> Self {
        self.params.insert("record".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_sample(mut self, val: ChopRecordSample) -> Self {
        self.params.insert("sample".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_sample_expr(mut self, expr: &str) -> Self {
        self.params.insert("sample".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_interp(mut self, val: ChopRecordInterp) -> Self {
        self.params.insert("interp".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_interp_expr(mut self, expr: &str) -> Self {
        self.params.insert("interp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_type(mut self, val: ChopRecordType) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_type_expr(mut self, expr: &str) -> Self {
        self.params.insert("type".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_output(mut self, val: ChopRecordOutput) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.params.insert("output".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_singleframetoggle(mut self, val: ChopRecordSingleframetoggle) -> Self {
        self.params.insert("singleframetoggle".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_singleframetoggle_expr(mut self, expr: &str) -> Self {
        self.params.insert("singleframetoggle".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_init(mut self, val: ChopRecordInit) -> Self {
        self.params.insert("init".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_init_expr(mut self, expr: &str) -> Self {
        self.params.insert("init".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_read(mut self, val: ChopRecordRead) -> Self {
        self.params.insert("read".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_read_expr(mut self, expr: &str) -> Self {
        self.params.insert("read".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_limita(mut self, val: ChopRecordLimita) -> Self {
        self.params.insert("limita".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_limita_expr(mut self, expr: &str) -> Self {
        self.params.insert("limita".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_limitb(mut self, val: ChopRecordLimitb) -> Self {
        self.params.insert("limitb".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_limitb_expr(mut self, expr: &str) -> Self {
        self.params.insert("limitb".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srselect(mut self, val: ChopRecordSrselect) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_units(mut self, val: ChopRecordUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_ipos(mut self, val: &str) -> Self {
        self.params.insert("ipos".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ipos_expr(mut self, expr: &str) -> Self {
        self.params.insert("ipos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_ispeed(mut self, val: &str) -> Self {
        self.params.insert("ispeed".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_ispeed_expr(mut self, expr: &str) -> Self {
        self.params.insert("ispeed".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ChopRecord {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "record"
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
pub enum ChopRenameSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopRenameUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopRename {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopRename {
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

    /// Connects to input 0: "Source"
    pub fn set_input_source<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source" and specifies the output index of the target node.
    pub fn set_input_source_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Name Reference"
    pub fn set_input_name_reference<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Name Reference" and specifies the output index of the target node.
    pub fn set_input_name_reference_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_srselect(mut self, val: ChopRenameSrselect) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_units(mut self, val: ChopRenameUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_renamefrom(mut self, val: &str) -> Self {
        self.params.insert("renamefrom".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_renamefrom_expr(mut self, expr: &str) -> Self {
        self.params.insert("renamefrom".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_renameto(mut self, val: &str) -> Self {
        self.params.insert("renameto".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_renameto_expr(mut self, expr: &str) -> Self {
        self.params.insert("renameto".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ChopRename {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "rename"
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
pub enum ChopReorderMethod {
    NumericPattern = 0,
    CharacterPattern = 1,
    BaseNameSort = 2,
    NumericSuffixSort = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopReorderRempos {
    AtBeginning = 0,
    AtEnding = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopReorderRemorder {
    SameAsInput = 0,
    Alphanumeric = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopReorderSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopReorderUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopReorder {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopReorder {
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

    /// Connects to input 0: "Source"
    pub fn set_input_source<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source" and specifies the output index of the target node.
    pub fn set_input_source_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Order Reference"
    pub fn set_input_order_reference<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Order Reference" and specifies the output index of the target node.
    pub fn set_input_order_reference_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: ChopReorderMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_rempos(mut self, val: ChopReorderRempos) -> Self {
        self.params.insert("rempos".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_rempos_expr(mut self, expr: &str) -> Self {
        self.params.insert("rempos".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_remorder(mut self, val: ChopReorderRemorder) -> Self {
        self.params.insert("remorder".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_remorder_expr(mut self, expr: &str) -> Self {
        self.params.insert("remorder".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srselect(mut self, val: ChopReorderSrselect) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_units(mut self, val: ChopReorderUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_numpattern(mut self, val: &str) -> Self {
        self.params.insert("numpattern".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_numpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert("numpattern".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_charpattern(mut self, val: &str) -> Self {
        self.params.insert("charpattern".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_charpattern_expr(mut self, expr: &str) -> Self {
        self.params.insert("charpattern".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ChopReorder {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "reorder"
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
pub enum ChopResampleMethod {
    /// Same Rate, New Interval
    SameRateNewInterval = 0,
    /// New Rate, Same Time Range
    NewRateSameTimeRange = 1,
    /// New Rate, Same Index Range
    NewRateSameIndexRange = 2,
    /// New Rate, New Interval
    NewRateNewInterval = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopResampleRelative {
    Absolute = 0,
    /// Relative to Start/End
    RelativeToStartEnd = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopResampleInterp {
    NoInterpolation = 0,
    Linear = 1,
    Cubic = 2,
    PulsePreserve = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopResampleSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopResampleUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopResample {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopResample {
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

    /// Connects to input 0: "Source"
    pub fn set_input_source<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Source" and specifies the output index of the target node.
    pub fn set_input_source_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Start/End Reference"
    pub fn set_input_start_end_reference<N: crate::core::types::HoudiniNode>(mut self, target: &N) -> Self {
        self.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Start/End Reference" and specifies the output index of the target node.
    pub fn set_input_start_end_reference_from<N: crate::core::types::HoudiniNode>(mut self, target: &N, output_index: usize) -> Self {
        self.inputs.insert(1, (target.get_id(), output_index));
        self
    }


    // --- Float parameters ---
    pub fn with_rate(mut self, val: f32) -> Self {
        self.params.insert("rate".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert("rate".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_start(mut self, val: f32) -> Self {
        self.params.insert("start".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert("start".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_end(mut self, val: f32) -> Self {
        self.params.insert("end".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.params.insert("end".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_cyclelen(mut self, val: f32) -> Self {
        self.params.insert("cyclelen".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_cyclelen_expr(mut self, expr: &str) -> Self {
        self.params.insert("cyclelen".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Float(val));
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolorstep".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert("gcolor".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: ChopResampleMethod) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert("method".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_relative(mut self, val: ChopResampleRelative) -> Self {
        self.params.insert("relative".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_relative_expr(mut self, expr: &str) -> Self {
        self.params.insert("relative".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_interp(mut self, val: ChopResampleInterp) -> Self {
        self.params.insert("interp".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_interp_expr(mut self, expr: &str) -> Self {
        self.params.insert("interp".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_srselect(mut self, val: ChopResampleSrselect) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert("srselect".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_units(mut self, val: ChopResampleUnits) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert("units".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert("scope".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert("export".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_constarea(mut self, val: bool) -> Self {
        self.params.insert("constarea".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_constarea_expr(mut self, expr: &str) -> Self {
        self.params.insert("constarea".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_correct(mut self, val: bool) -> Self {
        self.params.insert("correct".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_correct_expr(mut self, expr: &str) -> Self {
        self.params.insert("correct".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.params.insert("timeslice".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.params.insert("unload".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ChopResample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "resample"
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
pub enum ChopRopChannelTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone)]
pub struct ChopRopChannel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopRopChannel {
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
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert("renderdialog".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float3 parameters ---
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert("f".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Menu parameters ---
    pub fn with_trange(mut self, val: ChopRopChannelTrange) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Menu(val as i32));
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert("trange".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- String parameters ---
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert("take".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_choppath(mut self, val: &str) -> Self {
        self.params.insert("choppath".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_choppath_expr(mut self, expr: &str) -> Self {
        self.params.insert("choppath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_chopoutput(mut self, val: &str) -> Self {
        self.params.insert("chopoutput".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_chopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert("chopoutput".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("prerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("preframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("postframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("postrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::String(val.to_string()));
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("lpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }

    // --- Toggle parameters ---
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert("mkpath".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert("mkpath".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert("initsim".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert("initsim".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tprerender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpreframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostframe".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Toggle(val));
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert("tpostrender".to_string(), crate::core::types::ParamValue::Expression(expr.to_string()));
        self
    }
}

impl crate::core::types::HoudiniNode for ChopRopChannel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "rop_channel"
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
pub struct ChopRopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, crate::core::types::ParamValue>,
    pub spare_params: Vec<crate::core::types::SpareParam>,
}

impl ChopRopnet {
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



    // --- Button parameters ---
    pub fn trigger_execute(mut self) -> Self {
        self.params.insert("execute".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert("renderdialog".to_string(), crate::core::types::ParamValue::Button);
        self
    }
}

impl crate::core::types::HoudiniNode for ChopRopnet {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "ropnet"
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
