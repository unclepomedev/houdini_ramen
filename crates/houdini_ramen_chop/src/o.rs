#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectCompute {
    Position = 0,
    Rotation = 1,
    Bearing = 2,
    SingleBearingAngle = 3,
    Distance = 4,
    InverseSquareDistance = 5,
    PositionAndRotation = 6,
    FullTransform = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectBearingref {
    XAxis = 0,
    YAxis = 1,
    ZAxis = 2,
    BearingVector = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectNameformat {
    ChannelName = 0,
    TargetAndChannelNames = 1,
    ReferenceAndChannelNames = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectLeft {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectRight {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopObject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopObject {
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

    pub fn set_xyz_target_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_xyz_reference_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input2".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_samplerate(mut self, val: f32) -> Self {
        self.params.insert(
            "samplerate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_samplerate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplerate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_start(mut self, val: f32) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_end(mut self, val: f32) -> Self {
        self.params.insert(
            "end".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "end".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defval(mut self, val: f32) -> Self {
        self.params.insert(
            "defval".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defval".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bearing(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bearing".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bearing_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bearing".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_compute(mut self, val: ChopObjectCompute) -> Self {
        self.params.insert(
            "compute".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compute_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compute".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: ChopObjectRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bearingref(mut self, val: ChopObjectBearingref) -> Self {
        self.params.insert(
            "bearingref".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bearingref_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bearingref".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameformat(mut self, val: ChopObjectNameformat) -> Self {
        self.params.insert(
            "nameformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_nameformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_range(mut self, val: ChopObjectRange) -> Self {
        self.params.insert(
            "range".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_left(mut self, val: ChopObjectLeft) -> Self {
        self.params.insert(
            "left".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_right(mut self, val: ChopObjectRight) -> Self {
        self.params.insert(
            "right".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopObjectSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_units(mut self, val: ChopObjectUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpath(mut self, val: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_referencepath(mut self, val: &str) -> Self {
        self.params.insert(
            "referencepath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_referencepath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "referencepath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tscope(mut self, val: &str) -> Self {
        self.params.insert(
            "tscope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tscope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tscope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usereflength(mut self, val: bool) -> Self {
        self.params.insert(
            "usereflength".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usereflength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usereflength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopObject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "object"
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

pub trait ChopObjectOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopObjectOutputs for ChopObject {}
impl ChopObjectOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopObject> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectchainNameformat {
    ChannelName = 0,
    OpAndChannelName = 1,
    FullPathAndChannelName = 2,
    RelativePathAndChannelName = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectchainRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectchainLeft {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectchainRight {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectchainSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjectchainUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopObjectchain {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopObjectchain {
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

    pub fn with_start(mut self, val: f32) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_end(mut self, val: f32) -> Self {
        self.params.insert(
            "end".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "end".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rate(mut self, val: f32) -> Self {
        self.params.insert(
            "rate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defval(mut self, val: f32) -> Self {
        self.params.insert(
            "defval".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defval".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_nameformat(mut self, val: ChopObjectchainNameformat) -> Self {
        self.params.insert(
            "nameformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_nameformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nameformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_range(mut self, val: ChopObjectchainRange) -> Self {
        self.params.insert(
            "range".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_left(mut self, val: ChopObjectchainLeft) -> Self {
        self.params.insert(
            "left".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "left".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_right(mut self, val: ChopObjectchainRight) -> Self {
        self.params.insert(
            "right".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_right_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "right".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopObjectchainSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_units(mut self, val: ChopObjectchainUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_startpath(mut self, val: &str) -> Self {
        self.params.insert(
            "startpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_startpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_endpath(mut self, val: &str) -> Self {
        self.params.insert(
            "endpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_endpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "endpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pretransformname(mut self, val: &str) -> Self {
        self.params.insert(
            "pretransformname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pretransformname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pretransformname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useendobject(mut self, val: bool) -> Self {
        self.params.insert(
            "useendobject".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useendobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useendobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_includerootsubnet(mut self, val: bool) -> Self {
        self.params.insert(
            "includerootsubnet".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_includerootsubnet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "includerootsubnet".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fetchpretransform(mut self, val: bool) -> Self {
        self.params.insert(
            "fetchpretransform".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fetchpretransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fetchpretransform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fetchbonerest(mut self, val: bool) -> Self {
        self.params.insert(
            "fetchbonerest".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fetchbonerest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fetchbonerest".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fetchboneconstraint(mut self, val: bool) -> Self {
        self.params.insert(
            "fetchboneconstraint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fetchboneconstraint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fetchboneconstraint".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fetchlocal(mut self, val: bool) -> Self {
        self.params.insert(
            "fetchlocal".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fetchlocal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fetchlocal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopObjectchain {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "objectchain"
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

pub trait ChopObjectchainOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopObjectchainOutputs for ChopObjectchain {}
impl ChopObjectchainOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopObjectchain> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjnetXord {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjnetRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopObjnetPreXform {
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
pub enum ChopObjnetUparmtype {
    Uniform = 0,
    ArcLength = 1,
}

#[derive(Debug, Clone)]
pub struct ChopObjnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopObjnet {
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

    pub fn with_scale(mut self, val: f32) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_scale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roll".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pos(mut self, val: f32) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bank(mut self, val: f32) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bank_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bank".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "s".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_up(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_up_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "up".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathorient(mut self, val: i32) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_pathorient_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathorient".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_display(mut self, val: i32) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_xord(mut self, val: ChopObjnetXord) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_xord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "xOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rord(mut self, val: ChopObjnetRord) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rOrd".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pre_xform(mut self, val: ChopObjnetPreXform) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_pre_xform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pre_xform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uparmtype(mut self, val: ChopObjnetUparmtype) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uparmtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookatpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookatpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookupobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookupobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup(mut self, val: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lookup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lookup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath(mut self, val: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pathobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pathobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label1(mut self, val: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label2(mut self, val: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label3(mut self, val: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label4(mut self, val: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_label4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "label4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputobj(mut self, val: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputobj_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputobj".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibleobjects(mut self, val: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_visibleobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "visibleobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript(mut self, val: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pickscript_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pickscript".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_keeppos(mut self, val: bool) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keeppos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "keeppos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_childcomp(mut self, val: bool) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_childcomp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "childcomp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraints_on".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tdisplay(mut self, val: bool) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tdisplay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tdisplay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_picking(mut self, val: bool) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_picking_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "picking".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_caching(mut self, val: bool) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_caching_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "caching".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_dcolor(mut self, val: bool) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_dcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_dcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopObjnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "objnet"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopOscillatorWavetype {
    Sine = 0,
    Gaussian = 1,
    Triangle = 2,
    Ramp = 3,
    Square = 4,
    Pulse = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopOscillatorSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopOscillatorUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopOscillator {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopOscillator {
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

    pub fn set_pitch_control_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_reset_pulse_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input2".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(1),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input2<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(1));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_playback_source_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input3".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(2),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input3<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(2));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_frequency(mut self, val: f32) -> Self {
        self.params.insert(
            "frequency".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_frequency_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "frequency".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_octave(mut self, val: f32) -> Self {
        self.params.insert(
            "octave".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_octave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "octave".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_amp(mut self, val: f32) -> Self {
        self.params.insert(
            "amp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_amp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "amp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bias(mut self, val: f32) -> Self {
        self.params.insert(
            "bias".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bias".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_phase(mut self, val: f32) -> Self {
        self.params.insert(
            "phase".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_phase_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "phase".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rate(mut self, val: f32) -> Self {
        self.params.insert(
            "rate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wavetype(mut self, val: ChopOscillatorWavetype) -> Self {
        self.params.insert(
            "wavetype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_wavetype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavetype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopOscillatorSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_units(mut self, val: ChopOscillatorUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_smooth(mut self, val: bool) -> Self {
        self.params.insert(
            "smooth".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_smooth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "smooth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopOscillator {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "oscillator"
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

pub trait ChopOscillatorOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopOscillatorOutputs for ChopOscillator {}
impl ChopOscillatorOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopOscillator> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopOutputSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopOutputUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopOutput {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopOutput {
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

    pub fn set_input_1_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_outputidx(mut self, val: i32) -> Self {
        self.params.insert(
            "outputidx".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_outputidx_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "outputidx".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopOutputSrselect) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "srselect".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_units(mut self, val: ChopOutputUnits) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "units".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "scope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "export".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for ChopOutput {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "output"
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

pub trait ChopOutputOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopOutputOutputs for ChopOutput {}
impl ChopOutputOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopOutput> {}
