#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathPreop {
    Off = 0,
    Negate = 1,
    Positive = 2,
    Root = 3,
    Square = 4,
    Inverse = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathChanop {
    Off = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Average = 5,
    Minimum = 6,
    Maximum = 7,
    Length = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathChopop {
    Off = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Average = 5,
    Minimum = 6,
    Maximum = 7,
    Length = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathPostop {
    Off = 0,
    Negate = 1,
    Positive = 2,
    Root = 3,
    Square = 4,
    Inverse = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathMatch {
    ChannelNumber = 0,
    ChannelName = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathMatchfailure {
    Error = 0,
    Warning = 1,
    Ignore = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathAlign {
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
pub enum ChopMathSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMathUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopMath {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl ChopMath {
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

    pub fn add_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(self.next_input_index),
            (out.node_id, out.pin),
        );
        self.next_input_index += 1;
        self
    }

    pub fn with_preoff(mut self, val: f32) -> Self {
        self.params.insert(
            "preoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_preoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_gain(mut self, val: f32) -> Self {
        self.params.insert(
            "gain".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gain_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gain".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postoff(mut self, val: f32) -> Self {
        self.params.insert(
            "postoff".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_postoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postoff".to_string(),
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
    pub fn with_fromrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "fromrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_fromrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fromrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_torange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "torange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_torange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "torange".to_string(),
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
    pub fn with_preop(mut self, val: ChopMathPreop) -> Self {
        self.params.insert(
            "preop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_preop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_chanop(mut self, val: ChopMathChanop) -> Self {
        self.params.insert(
            "chanop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_chanop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chanop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_chopop(mut self, val: ChopMathChopop) -> Self {
        self.params.insert(
            "chopop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_chopop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "chopop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postop(mut self, val: ChopMathPostop) -> Self {
        self.params.insert(
            "postop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_postop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_match(mut self, val: ChopMathMatch) -> Self {
        self.params.insert(
            "match".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_match_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "match".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_matchfailure(mut self, val: ChopMathMatchfailure) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchfailure_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "matchfailure".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_align(mut self, val: ChopMathAlign) -> Self {
        self.params.insert(
            "align".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_align_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "align".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopMathSrselect) -> Self {
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
    pub fn with_units(mut self, val: ChopMathUnits) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for ChopMath {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "math"
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

pub trait ChopMathOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopMathOutputs for ChopMath {}
impl ChopMathOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopMath> {}

#[derive(Debug, Clone)]
pub struct ChopMatnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopMatnet {
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
}

impl houdini_ramen_core::types::HoudiniNode for ChopMatnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "matnet"
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
pub enum ChopMergeAlign {
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
pub enum ChopMergeDuplicate {
    /// Duplicate. Make Unique Names
    DuplicateMakeUniqueNames = 0,
    /// Duplicate. Keep First
    DuplicateKeepFirst = 1,
    /// Duplicate. Keep Last
    DuplicateKeepLast = 2,
    OverrideExistingChannels = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMergeSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMergeUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopMerge {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl ChopMerge {
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

    pub fn add_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(self.next_input_index),
            (out.node_id, out.pin),
        );
        self.next_input_index += 1;
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
    pub fn with_align(mut self, val: ChopMergeAlign) -> Self {
        self.params.insert(
            "align".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_align_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "align".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_duplicate(mut self, val: ChopMergeDuplicate) -> Self {
        self.params.insert(
            "duplicate".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_duplicate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "duplicate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopMergeSrselect) -> Self {
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
    pub fn with_units(mut self, val: ChopMergeUnits) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for ChopMerge {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "merge"
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

pub trait ChopMergeOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopMergeOutputs for ChopMerge {}
impl ChopMergeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopMerge> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinRecordtype {
    SingleFrame = 0,
    CurrentFrame = 1,
    CurrentTimeSlice = 2,
    FullLength = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinNotemeth {
    OneMultiplexedChannel = 0,
    SeparateChannels = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinVelocity {
    Off = 0,
    NoteAmplitude = 1,
    SeparateChannels = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinNotenorm {
    None = 0,
    /// 0 to 1
    N0To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinControltype {
    ByIndexOnly = 0,
    BankSelect = 1,
    ModulationWheel = 2,
    BreathControl = 3,
    FootController = 4,
    PortamentoTime = 5,
    DataEntry = 6,
    ChannelVolume = 7,
    Balance = 8,
    Pan = 9,
    ExpressionControl = 10,
    /// Effect Control 1-2
    EffectControl1Minus2 = 11,
    /// 14 bit General Purpose 1-4
    N14BitGeneralPurpose1Minus4 = 12,
    /// Sustain on/off
    SustainOnOff = 13,
    /// Portamento on/off
    PortamentoOnOff = 14,
    /// Sustenuto on/off
    SustenutoOnOff = 15,
    /// Soft Pedal on/off
    SoftPedalOnOff = 16,
    /// Legato Footswitch on/off
    LegatoFootswitchOnOff = 17,
    /// Hold 2 on/off
    Hold2OnOff = 18,
    /// Sound Controller 1-10
    SoundController1Minus10 = 19,
    /// 7 bit General Purpose 5-8
    N7BitGeneralPurpose5Minus8 = 20,
    PortamentoControl = 21,
    /// Effect Depth 1-5
    EffectDepth1Minus5 = 22,
    /// Non-Registered Parameter
    NonMinusRegisteredParameter = 23,
    RegisteredParameter = 24,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinFormat {
    /// 7 bit Controllers
    N7BitControllers = 0,
    /// 14 bit Controllers
    N14BitControllers = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinNorm {
    None = 0,
    /// 0 to 1
    N0To1 = 1,
    /// -1 to 1
    Minus1To1 = 2,
    /// On/Off
    OnOff = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinLeft {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinRight {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidiinUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopMidiin {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopMidiin {
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

    pub fn trigger_reset(mut self) -> Self {
        self.params.insert(
            "reset".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
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
    pub fn with_ticks(mut self, val: i32) -> Self {
        self.params.insert(
            "ticks".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_ticks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ticks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_recordtype(mut self, val: ChopMidiinRecordtype) -> Self {
        self.params.insert(
            "recordtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_recordtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "recordtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notemeth(mut self, val: ChopMidiinNotemeth) -> Self {
        self.params.insert(
            "notemeth".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_notemeth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notemeth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_velocity(mut self, val: ChopMidiinVelocity) -> Self {
        self.params.insert(
            "velocity".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_velocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notenorm(mut self, val: ChopMidiinNotenorm) -> Self {
        self.params.insert(
            "notenorm".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_notenorm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notenorm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controltype(mut self, val: ChopMidiinControltype) -> Self {
        self.params.insert(
            "controltype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_controltype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "controltype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_format(mut self, val: ChopMidiinFormat) -> Self {
        self.params.insert(
            "format".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_format_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "format".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_norm(mut self, val: ChopMidiinNorm) -> Self {
        self.params.insert(
            "norm".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_norm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "norm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_left(mut self, val: ChopMidiinLeft) -> Self {
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
    pub fn with_right(mut self, val: ChopMidiinRight) -> Self {
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
    pub fn with_srselect(mut self, val: ChopMidiinSrselect) -> Self {
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
    pub fn with_units(mut self, val: ChopMidiinUnits) -> Self {
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
    pub fn with_source(mut self, val: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_source_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "source".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_channel(mut self, val: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefix(mut self, val: &str) -> Self {
        self.params.insert(
            "prefix".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notename(mut self, val: &str) -> Self {
        self.params.insert(
            "notename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notescope(mut self, val: &str) -> Self {
        self.params.insert(
            "notescope".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notescope_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notescope".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_velname(mut self, val: &str) -> Self {
        self.params.insert(
            "velname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_velname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aftername(mut self, val: &str) -> Self {
        self.params.insert(
            "aftername".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aftername_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aftername".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pressname(mut self, val: &str) -> Self {
        self.params.insert(
            "pressname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pressname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pitchname(mut self, val: &str) -> Self {
        self.params.insert(
            "pitchname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pitchname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pitchname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controlname(mut self, val: &str) -> Self {
        self.params.insert(
            "controlname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controlname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "controlname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controlind(mut self, val: &str) -> Self {
        self.params.insert(
            "controlind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controlind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "controlind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_progname(mut self, val: &str) -> Self {
        self.params.insert(
            "progname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_progname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "progname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pulsename(mut self, val: &str) -> Self {
        self.params.insert(
            "pulsename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pulsename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pulsename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rampname(mut self, val: &str) -> Self {
        self.params.insert(
            "rampname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rampname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rampname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rampstats(mut self, val: &str) -> Self {
        self.params.insert(
            "rampstats".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rampstats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rampstats".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barname(mut self, val: &str) -> Self {
        self.params.insert(
            "barname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barstats(mut self, val: &str) -> Self {
        self.params.insert(
            "barstats".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barstats_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barstats".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barmsg(mut self, val: &str) -> Self {
        self.params.insert(
            "barmsg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barmsg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barmsg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_songpos(mut self, val: &str) -> Self {
        self.params.insert(
            "songpos".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_songpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "songpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname1(mut self, val: &str) -> Self {
        self.params.insert(
            "exname1".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg1(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg1".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname2(mut self, val: &str) -> Self {
        self.params.insert(
            "exname2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg2(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg2".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname3(mut self, val: &str) -> Self {
        self.params.insert(
            "exname3".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg3(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg3".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg3_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg3".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname4(mut self, val: &str) -> Self {
        self.params.insert(
            "exname4".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg4(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg4".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg4_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg4".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname5(mut self, val: &str) -> Self {
        self.params.insert(
            "exname5".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg5(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg5".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg5_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg5".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname6(mut self, val: &str) -> Self {
        self.params.insert(
            "exname6".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg6(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg6".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg6_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg6".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname7(mut self, val: &str) -> Self {
        self.params.insert(
            "exname7".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg7(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg7".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg7_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg7".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname8(mut self, val: &str) -> Self {
        self.params.insert(
            "exname8".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg8(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg8".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg8_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg8".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname9(mut self, val: &str) -> Self {
        self.params.insert(
            "exname9".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname9".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg9(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg9".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg9_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg9".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname10(mut self, val: &str) -> Self {
        self.params.insert(
            "exname10".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname10".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg10(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg10".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg10_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg10".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname11(mut self, val: &str) -> Self {
        self.params.insert(
            "exname11".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname11".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg11(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg11".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg11_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg11".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname12(mut self, val: &str) -> Self {
        self.params.insert(
            "exname12".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname12".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg12(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg12".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg12_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg12".to_string(),
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
    pub fn with_entire(mut self, val: bool) -> Self {
        self.params.insert(
            "entire".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_entire_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "entire".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_echo(mut self, val: bool) -> Self {
        self.params.insert(
            "echo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_echo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "echo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_record(mut self, val: bool) -> Self {
        self.params.insert(
            "record".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_record_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "record".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unwrap(mut self, val: bool) -> Self {
        self.params.insert(
            "unwrap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unwrap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unwrap".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ChopMidiin {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "midiin"
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

pub trait ChopMidiinOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopMidiinOutputs for ChopMidiin {}
impl ChopMidiinOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopMidiin> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutRecover {
    PlayMissedEventsImmediately = 0,
    DropMissedEvents = 1,
    DelayAllEvents = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutAutonoteoff {
    None = 0,
    AtPlaybackStart = 1,
    AtPlaybackEnd = 2,
    AtPlaybackStartAndEnd = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutNotenorm {
    None = 0,
    /// 0 to 1
    N0To1 = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutFormat {
    /// 7 bit Controllers
    N7BitControllers = 0,
    /// 14 bit Controllers
    N14BitControllers = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutNorm {
    None = 0,
    /// 0 to 1
    N0To1 = 1,
    /// -1 to 1
    Minus1To1 = 2,
    /// On/Off
    OnOff = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutExformat {
    /// No Channel; Static Message
    NoChannelStaticMessage = 0,
    /// Single Channel, Single Sample Message
    SingleChannelSingleSampleMessage = 1,
    /// Single Channel, Multi Sample Message
    SingleChannelMultiSampleMessage = 2,
    MultiChannelMessage = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMidioutUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopMidiout {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopMidiout {
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

    pub fn trigger_entire(mut self) -> Self {
        self.params.insert(
            "entire".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_sendall(mut self) -> Self {
        self.params.insert(
            "sendall".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_reset(mut self) -> Self {
        self.params.insert(
            "reset".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_prequeue(mut self, val: f32) -> Self {
        self.params.insert(
            "prequeue".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_prequeue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prequeue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_delay(mut self, val: f32) -> Self {
        self.params.insert(
            "delay".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_delay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "delay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxjump(mut self, val: f32) -> Self {
        self.params.insert(
            "maxjump".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxjump_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxjump".to_string(),
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
    pub fn with_barticks(mut self, val: i32) -> Self {
        self.params.insert(
            "barticks".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_barticks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barticks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exlength(mut self, val: i32) -> Self {
        self.params.insert(
            "exlength".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_exlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_recover(mut self, val: ChopMidioutRecover) -> Self {
        self.params.insert(
            "recover".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_recover_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "recover".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autonoteoff(mut self, val: ChopMidioutAutonoteoff) -> Self {
        self.params.insert(
            "autonoteoff".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_autonoteoff_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autonoteoff".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notenorm(mut self, val: ChopMidioutNotenorm) -> Self {
        self.params.insert(
            "notenorm".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_notenorm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notenorm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_format(mut self, val: ChopMidioutFormat) -> Self {
        self.params.insert(
            "format".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_format_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "format".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_norm(mut self, val: ChopMidioutNorm) -> Self {
        self.params.insert(
            "norm".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_norm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "norm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exformat(mut self, val: ChopMidioutExformat) -> Self {
        self.params.insert(
            "exformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_exformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopMidioutSrselect) -> Self {
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
    pub fn with_units(mut self, val: ChopMidioutUnits) -> Self {
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
    pub fn with_destination(mut self, val: &str) -> Self {
        self.params.insert(
            "destination".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_destination_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "destination".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file(mut self, val: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "file".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_channel(mut self, val: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_channel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "channel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefix(mut self, val: &str) -> Self {
        self.params.insert(
            "prefix".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prefix_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prefix".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notename(mut self, val: &str) -> Self {
        self.params.insert(
            "notename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_notename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "notename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_velname(mut self, val: &str) -> Self {
        self.params.insert(
            "velname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_velname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "velname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aftername(mut self, val: &str) -> Self {
        self.params.insert(
            "aftername".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_aftername_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "aftername".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pressname(mut self, val: &str) -> Self {
        self.params.insert(
            "pressname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pressname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pitchname(mut self, val: &str) -> Self {
        self.params.insert(
            "pitchname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pitchname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pitchname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controlname(mut self, val: &str) -> Self {
        self.params.insert(
            "controlname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_controlname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "controlname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_progname(mut self, val: &str) -> Self {
        self.params.insert(
            "progname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_progname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "progname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barname(mut self, val: &str) -> Self {
        self.params.insert(
            "barname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_barname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "barname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname(mut self, val: &str) -> Self {
        self.params.insert(
            "exname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exdata(mut self, val: &str) -> Self {
        self.params.insert(
            "exdata".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exdata".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg(mut self, val: &str) -> Self {
        self.params.insert(
            "exmsg".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_exmsg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "exmsg".to_string(),
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
    pub fn with_echo(mut self, val: bool) -> Self {
        self.params.insert(
            "echo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_echo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "echo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_startstop(mut self, val: bool) -> Self {
        self.params.insert(
            "startstop".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_startstop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "startstop".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ChopMidiout {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "midiout"
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

pub trait ChopMidioutOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopMidioutOutputs for ChopMidiout {}
impl ChopMidioutOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopMidiout> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouseActive {
    Off = 0,
    On = 1,
    WhilePlaying = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouseLeft {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouseRight {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouseSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouseUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopMouse {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopMouse {
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
    pub fn with_active(mut self, val: ChopMouseActive) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_active_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_left(mut self, val: ChopMouseLeft) -> Self {
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
    pub fn with_right(mut self, val: ChopMouseRight) -> Self {
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
    pub fn with_srselect(mut self, val: ChopMouseSrselect) -> Self {
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
    pub fn with_units(mut self, val: ChopMouseUnits) -> Self {
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
    pub fn with_posxname(mut self, val: &str) -> Self {
        self.params.insert(
            "posxname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_posxname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posxname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_posyname(mut self, val: &str) -> Self {
        self.params.insert(
            "posyname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_posyname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "posyname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pressurename(mut self, val: &str) -> Self {
        self.params.insert(
            "pressurename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_pressurename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "pressurename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_anglename(mut self, val: &str) -> Self {
        self.params.insert(
            "anglename".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_anglename_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "anglename".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tiltname(mut self, val: &str) -> Self {
        self.params.insert(
            "tiltname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tiltname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tiltname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rollname(mut self, val: &str) -> Self {
        self.params.insert(
            "rollname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rollname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rollname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cursorname(mut self, val: &str) -> Self {
        self.params.insert(
            "cursorname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cursorname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cursorname".to_string(),
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
    pub fn with_usetablet(mut self, val: bool) -> Self {
        self.params.insert(
            "usetablet".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetablet_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetablet".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ChopMouse {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "mouse"
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

pub trait ChopMouseOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopMouseOutputs for ChopMouse {}
impl ChopMouseOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopMouse> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouse3dRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouse3dLeft {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouse3dRight {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouse3dSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMouse3dUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopMouse3d {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopMouse3d {
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

    pub fn trigger_setfocus(mut self) -> Self {
        self.params.insert(
            "setfocus".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_releasefocus(mut self) -> Self {
        self.params.insert(
            "releasefocus".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
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
    pub fn with_range(mut self, val: ChopMouse3dRange) -> Self {
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
    pub fn with_left(mut self, val: ChopMouse3dLeft) -> Self {
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
    pub fn with_right(mut self, val: ChopMouse3dRight) -> Self {
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
    pub fn with_srselect(mut self, val: ChopMouse3dSrselect) -> Self {
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
    pub fn with_units(mut self, val: ChopMouse3dUnits) -> Self {
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
    pub fn with_axes(mut self, val: &str) -> Self {
        self.params.insert(
            "axes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_btns(mut self, val: &str) -> Self {
        self.params.insert(
            "btns".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_btns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "btns".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ChopMouse3d {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "mouse3d"
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

pub trait ChopMouse3dOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopMouse3dOutputs for ChopMouse3d {}
impl ChopMouse3dOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopMouse3d> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMultiplyVexAlign {
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
pub enum ChopMultiplyVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMultiplyVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMultiplySrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopMultiplyUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopMultiply {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl ChopMultiply {
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
    pub fn set_input_2_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_pivot_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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
    pub fn set_sub_network_input_4_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input4".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(3),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input4<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(3));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input4".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_5_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input5".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(4),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input5<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(4));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input5".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_6_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input6".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(5),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input6<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(5));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input6".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_7_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input7".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(6),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input7<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(6));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input7".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_8_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input8".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(7),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input8<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(7));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input8".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_9_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input9".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(8),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input9<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(8));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input9".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_10_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input10".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(9),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input10<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(9));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input10".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_11_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input11".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(10),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input11<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(10));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input11".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_12_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input12".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(11),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input12<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(11));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input12".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_13_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input13".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(12),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input13<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(12));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input13".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_14_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input14".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(13),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input14<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(13));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input14".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_15_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input15".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(14),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input15<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(14));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input15".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_16_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input16".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(15),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input16<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(15));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input16".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_17_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input17".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(16),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input17<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(16));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input17".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_18_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input18".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(17),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input18<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(17));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input18".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_19_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input19".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(18),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input19<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(18));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input19".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_20_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input20".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(19),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input20<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(19));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input20".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_21_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input21".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(20),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input21<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(20));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input21".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_22_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input22".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(21),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input22<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(21));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input22".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_23_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input23".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(22),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input23<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(22));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input23".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_24_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input24".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(23),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input24<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(23));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input24".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_25_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input25".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(24),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input25<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(24));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input25".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_26_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input26".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(25),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input26<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(25));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input26".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_27_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input27".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(26),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input27<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(26));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input27".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_28_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input28".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(27),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input28<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(27));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input28".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_29_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input29".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(28),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input29<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(28));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input29".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_30_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input30".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(29),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input30<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(29));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input30".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_31_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input31".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(30),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input31<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(30));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input31".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_32_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input32".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(31),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input32<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(31));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input32".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_33_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input33".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(32),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input33<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(32));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input33".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_34_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input34".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(33),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input34<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(33));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input34".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_35_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input35".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(34),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input35<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(34));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input35".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_36_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input36".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(35),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input36<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(35));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input36".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_37_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input37".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(36),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input37<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(36));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input37".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_38_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input38".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(37),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input38<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(37));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input38".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_39_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input39".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(38),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input39<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(38));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input39".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_40_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input40".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(39),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input40<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(39));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input40".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_41_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input41".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(40),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input41<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(40));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input41".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_42_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input42".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(41),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input42<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(41));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input42".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_43_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input43".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(42),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input43<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(42));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input43".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_44_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input44".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(43),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input44<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(43));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input44".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_45_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input45".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(44),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input45<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(44));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input45".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_46_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input46".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(45),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input46<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(45));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input46".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_47_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input47".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(46),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input47<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(46));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input47".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_48_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input48".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(47),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input48<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(47));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input48".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_49_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input49".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(48),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input49<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(48));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input49".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_50_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input50".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(49),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input50<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(49));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input50".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_51_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input51".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(50),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input51<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(50));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input51".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_52_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input52".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(51),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input52<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(51));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input52".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_53_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input53".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(52),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input53<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(52));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input53".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_54_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input54".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(53),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input54<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(53));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input54".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_55_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input55".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(54),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input55<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(54));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input55".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_56_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input56".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(55),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input56<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(55));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input56".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_57_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input57".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(56),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input57<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(56));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input57".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_58_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input58".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(57),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input58<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(57));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input58".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_59_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input59".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(58),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input59<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(58));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input59".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_60_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input60".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(59),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input60<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(59));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input60".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_61_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input61".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(60),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input61<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(60));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input61".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_62_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input62".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(61),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input62<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(61));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input62".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_63_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input63".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(62),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input63<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(62));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input63".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_64_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input64".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(63),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input64<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(63));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input64".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_65_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input65".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(64),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input65<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(64));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input65".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_66_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input66".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(65),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input66<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(65));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input66".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_67_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input67".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(66),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input67<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(66));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input67".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_68_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input68".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(67),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input68<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(67));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input68".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_69_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input69".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(68),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input69<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(68));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input69".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_70_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input70".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(69),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input70<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(69));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input70".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_71_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input71".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(70),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input71<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(70));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input71".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_72_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input72".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(71),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input72<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(71));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input72".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_73_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input73".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(72),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input73<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(72));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input73".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_74_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input74".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(73),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input74<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(73));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input74".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_75_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input75".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(74),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input75<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(74));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input75".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_76_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input76".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(75),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input76<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(75));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input76".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_77_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input77".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(76),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input77<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(76));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input77".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_78_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input78".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(77),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input78<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(77));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input78".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_79_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input79".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(78),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input79<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(78));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input79".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_80_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input80".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(79),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input80<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(79));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input80".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_81_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input81".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(80),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input81<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(80));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input81".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_82_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input82".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(81),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input82<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(81));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input82".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_83_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input83".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(82),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input83<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(82));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input83".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_84_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input84".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(83),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input84<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(83));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input84".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_85_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input85".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(84),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input85<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(84));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input85".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_86_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input86".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(85),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input86<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(85));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input86".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_87_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input87".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(86),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input87<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(86));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input87".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_88_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input88".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(87),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input88<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(87));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input88".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_89_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input89".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(88),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input89<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(88));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input89".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_90_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input90".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(89),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input90<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(89));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input90".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_91_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input91".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(90),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input91<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(90));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input91".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_92_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input92".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(91),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input92<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(91));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input92".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_93_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input93".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(92),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input93<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(92));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input93".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_94_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input94".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(93),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input94<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(93));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input94".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_95_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input95".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(94),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input95<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(94));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input95".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_96_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input96".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(95),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input96<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(95));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input96".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_97_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input97".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(96),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input97<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(96));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input97".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_98_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input98".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(97),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input98<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(97));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input98".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_99_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input99".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(98),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input99<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(98));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input99".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_sub_network_input_100_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Name(
                "input100".to_string(),
            ));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Index(99),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_input_name_input100<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs
            .remove(&houdini_ramen_core::types::InputPin::Index(99));
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input100".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn trigger_vex_edit(mut self) -> Self {
        self.params.insert(
            "vex_edit".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.params.insert(
            "vex_reload".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.params.insert(
            "vex_start".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_start".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.params.insert(
            "vex_end".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_end".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.params.insert(
            "vex_rate".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_rate".to_string(),
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
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.params.insert(
            "__iterate_over_channels__".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "__iterate_over_channels__".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopMultiplyVexAlign) -> Self {
        self.params.insert(
            "vex_align".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_align".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopMultiplyVexRange) -> Self {
        self.params.insert(
            "vex_range".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopMultiplyVexNumThreads) -> Self {
        self.params.insert(
            "vex_num_threads".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_num_threads".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopMultiplySrselect) -> Self {
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
    pub fn with_units(mut self, val: ChopMultiplyUnits) -> Self {
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
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_name".to_string(),
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
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.params.insert(
            "vex_precision".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vex_precision".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for ChopMultiply {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "multiply"
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

pub trait ChopMultiplyOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Sub-Network Output #1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl ChopMultiplyOutputs for ChopMultiply {}
impl ChopMultiplyOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<ChopMultiply> {}
