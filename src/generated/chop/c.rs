#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelType {
    Float = 0,
    EulerRotation = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelRord {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
    UseValueAnimation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelLeft {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelRight {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopChannel {
    pub base: crate::core::types::NodeBase,
}

impl ChopChannel {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_copytoexport(mut self) -> Self {
        self.base.params.insert(
            "copytoexport".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_copyfromexport(mut self) -> Self {
        self.base.params.insert(
            "copyfromexport".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_updatealiases(mut self) -> Self {
        self.base.params.insert(
            "updatealiases".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_sort(mut self) -> Self {
        self.base
            .params
            .insert("sort".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_resetall(mut self) -> Self {
        self.base.params.insert(
            "resetall".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defval(mut self, val: f32) -> Self {
        self.base.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float4 parameters ---
    pub fn with_value_inst(mut self, index1: usize, val: [f32; 4]) -> Self {
        self.base.params.insert(
            format!("value{}", index1),
            crate::core::types::ParamValue::Float4(val),
        );
        self
    }
    pub fn with_value_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("value{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_size_inst(mut self, index1: usize, val: i32) -> Self {
        self.base.params.insert(
            format!("size{}", index1),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_size_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("size{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_type_inst(mut self, index1: usize, val: ChopChannelType) -> Self {
        self.base.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_type_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("type{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rord_inst(mut self, index1: usize, val: ChopChannelRord) -> Self {
        self.base.params.insert(
            format!("rOrd{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_rord_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("rOrd{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_range(mut self, val: ChopChannelRange) -> Self {
        self.base.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_left(mut self, val: ChopChannelLeft) -> Self {
        self.base.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right(mut self, val: ChopChannelRight) -> Self {
        self.base.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_right_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopChannelSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopChannelUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name_inst(mut self, index1: usize, val: &str) -> Self {
        self.base.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("name{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_showall(mut self, val: bool) -> Self {
        self.base.params.insert(
            "showall".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showall_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "showall".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_keypersample(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keypersample".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keypersample_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keypersample".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_adddeps(mut self, val: bool) -> Self {
        self.base.params.insert(
            "adddeps".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adddeps_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "adddeps".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.base.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("enable{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_exacttime(mut self, val: bool) -> Self {
        self.base.params.insert(
            "exacttime".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_exacttime_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "exacttime".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopChannel {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "channel"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelwrangleIterate {
    OverChannelsAndSamples = 0,
    OverSamplesAndChannels = 1,
    OverSamplesAndEveryNthChannels = 2,
    /// Over Samples and Every Translate/Rotate/Scale
    OverSamplesAndEveryTranslateRotateScale = 3,
    OverClip = 4,
    OverChannels = 5,
    OverSamples = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelwrangleVexAlign {
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
pub enum ChopChannelwrangleVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelwrangleVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelwrangleSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelwrangleUnits2 {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopChannelwrangleUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopChannelwrangle {
    pub base: crate::core::types::NodeBase,
}

impl ChopChannelwrangle {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3"
    pub fn set_input_sub_network_input_3(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4"
    pub fn set_input_sub_network_input_4(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_vex_count(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vex_count".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vex_count_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_count".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_channel_step(mut self, val: i32) -> Self {
        self.base.params.insert(
            "channel_step".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_channel_step_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "channel_step".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_iterate(mut self, val: ChopChannelwrangleIterate) -> Self {
        self.base.params.insert(
            "iterate".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_iterate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "iterate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopChannelwrangleVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopChannelwrangleVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopChannelwrangleVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopChannelwrangleSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units2(mut self, val: ChopChannelwrangleUnits2) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopChannelwrangleUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_bind_names(mut self, val: &str) -> Self {
        self.base.params.insert(
            "bind_names".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_bind_names_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "bind_names".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_snippet(mut self, val: &str) -> Self {
        self.base.params.insert(
            "snippet".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_snippet_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "snippet".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_cwdpath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_cwdpath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opmenu(mut self, val: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_vex_strict(mut self, val: bool) -> Self {
        self.base.params.insert(
            "vex_strict".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_vex_strict_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_strict".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_check_timedep_inputs(mut self, val: bool) -> Self {
        self.base.params.insert(
            "check_timedep_inputs".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_check_timedep_inputs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "check_timedep_inputs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice2(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice2".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopChannelwrangle {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "channelwrangle"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ChopChannelwrangleInnerExt {
    fn vopchop1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ChopChannelwrangleInnerExt for crate::core::graph::InnerGraph<'a, ChopChannelwrangle> {
    fn vopchop1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("vopchop1")
    }
}

#[derive(Debug, Clone)]
pub struct ChopChopnet {
    pub base: crate::core::types::NodeBase,
}

impl ChopChopnet {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }
}

impl crate::core::types::HoudiniNode for ChopChopnet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "chopnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCompMatch {
    ChannelNumber = 0,
    ChannelName = 1,
    ChannelUnion = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCompMatchfailure {
    Error = 0,
    Warning = 1,
    Ignore = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCompRelative {
    Absolute = 0,
    /// Relative to Start/End
    RelativeToStartEnd = 1,
    UseCurrentFrame = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCompRisefunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Cubic = 4,
    Add = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCompFallfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Cubic = 4,
    Add = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCompSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCompUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopComp {
    pub base: crate::core::types::NodeBase,
}

impl ChopComp {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Base Source"
    pub fn set_input_base_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Base Source" and specifies the output index of the target node.
    pub fn set_input_base_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Layer Source"
    pub fn set_input_layer_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Layer Source" and specifies the output index of the target node.
    pub fn set_input_layer_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Effect Source"
    pub fn set_input_effect_source(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Effect Source" and specifies the output index of the target node.
    pub fn set_input_effect_source_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_base(mut self, val: f32) -> Self {
        self.base.params.insert(
            "base".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_base_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "base".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cyclelen(mut self, val: f32) -> Self {
        self.base.params.insert(
            "cyclelen".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cyclelen_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cyclelen".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_effect(mut self, val: f32) -> Self {
        self.base.params.insert(
            "effect".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_effect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "effect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_peak(mut self, val: f32) -> Self {
        self.base.params.insert(
            "peak".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_peak_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "peak".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_release(mut self, val: f32) -> Self {
        self.base.params.insert(
            "release".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_release_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "release".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_match(mut self, val: ChopCompMatch) -> Self {
        self.base.params.insert(
            "match".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_match_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "match".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_matchfailure(mut self, val: ChopCompMatchfailure) -> Self {
        self.base.params.insert(
            "matchfailure".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_matchfailure_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "matchfailure".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_relative(mut self, val: ChopCompRelative) -> Self {
        self.base.params.insert(
            "relative".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_relative_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "relative".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_risefunc(mut self, val: ChopCompRisefunc) -> Self {
        self.base.params.insert(
            "risefunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_risefunc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "risefunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_fallfunc(mut self, val: ChopCompFallfunc) -> Self {
        self.base.params.insert(
            "fallfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_fallfunc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "fallfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopCompSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopCompUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_rotscope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "rotscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_rotscope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_quatrot(mut self, val: bool) -> Self {
        self.base.params.insert(
            "quatrot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_quatrot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "quatrot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_shortrot(mut self, val: bool) -> Self {
        self.base.params.insert(
            "shortrot".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_shortrot_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "shortrot".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopComp {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "comp"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstantLeft {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstantRight {
    Hold = 0,
    Slope = 1,
    Cycle = 2,
    Mirror = 3,
    DefaultValue = 4,
    CycleWithOffset = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstantSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstantUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstant {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstant {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Snap Channels"
    pub fn set_input_snap_channels(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Snap Channels" and specifies the output index of the target node.
    pub fn set_input_snap_channels_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Active"
    pub fn set_input_active(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Active" and specifies the output index of the target node.
    pub fn set_input_active_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_snap(mut self) -> Self {
        self.base
            .params
            .insert("snap".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_value0(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value0".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value0_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value0".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value1(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value2(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value3(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value4(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value5(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value5_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value6(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value6_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value7(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value7_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value8(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value8_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value9(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value9_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value10(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value10_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value11(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value11".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value11_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value12(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value12".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value12_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value13(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value13".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value13_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value14(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value14".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value14_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value15(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value15".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value15_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value16(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value16".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value16_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value17(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value17".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value17_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value17".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value18(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value18".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value18_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value18".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value19(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value19".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value19_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value19".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value20(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value20".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value20_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value20".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value21(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value21".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value21_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value21".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value22(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value22".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value22_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value22".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value23(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value23".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value23_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value23".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value24(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value24".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value24_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value24".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value25(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value25".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value25_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value25".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value26(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value26".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value26_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value26".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value27(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value27".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value27_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value27".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value28(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value28".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value28_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value28".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value29(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value29".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value29_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value29".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value30(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value30".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value30_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value30".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value31(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value31".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value31_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value31".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value32(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value32".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value32_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value32".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value33(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value33".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value33_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value33".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value34(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value34".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value34_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value34".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value35(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value35".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value35_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value35".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value36(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value36".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value36_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value36".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value37(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value37".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value37_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value37".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value38(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value38".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value38_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value38".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_value39(mut self, val: f32) -> Self {
        self.base.params.insert(
            "value39".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_value39_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "value39".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_defval(mut self, val: f32) -> Self {
        self.base.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_defval_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "defval".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_first(mut self, val: i32) -> Self {
        self.base.params.insert(
            "first".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_first_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "first".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_left(mut self, val: ChopConstantLeft) -> Self {
        self.base.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_left_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "left".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_right(mut self, val: ChopConstantRight) -> Self {
        self.base.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_right_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "right".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstantSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstantUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_name0(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name0".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name0_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name0".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name1(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name3(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name4(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name5(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name5_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name6(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name6_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name7(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name7_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name8(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name8_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name9(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name9_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name10(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name10_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name11(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name11".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name11_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name11".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name12(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name12".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name12_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name12".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name13(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name13".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name13_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name13".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name14(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name14".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name14_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name14".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name15(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name15".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name15_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name15".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name16(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name16".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name16_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name16".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name17(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name17".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name17_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name17".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name18(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name18".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name18_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name18".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name19(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name19".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name19_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name19".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name20(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name20".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name20_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name20".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name21(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name21".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name21_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name21".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name22(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name22".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name22_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name22".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name23(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name23".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name23_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name23".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name24(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name24".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name24_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name24".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name25(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name25".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name25_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name25".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name26(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name26".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name26_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name26".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name27(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name27".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name27_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name27".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name28(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name28".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name28_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name28".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name29(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name29".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name29_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name29".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name30(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name30".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name30_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name30".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name31(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name31".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name31_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name31".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name32(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name32".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name32_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name32".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name33(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name33".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name33_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name33".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name34(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name34".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name34_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name34".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name35(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name35".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name35_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name35".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name36(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name36".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name36_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name36".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name37(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name37".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name37_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name37".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name38(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name38".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name38_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name38".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_name39(mut self, val: &str) -> Self {
        self.base.params.insert(
            "name39".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_name39_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "name39".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_current(mut self, val: bool) -> Self {
        self.base.params.insert(
            "current".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_current_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "current".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_single(mut self, val: bool) -> Self {
        self.base.params.insert(
            "single".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_single_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "single".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstant {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constant"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintbeginMode {
    WorldTransform = 0,
    LocalTransform = 1,
    ParentTransform = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintbeginTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintbeginXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintbeginVexAlign {
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
pub enum ChopConstraintbeginVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintbeginSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintbeginUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintbeginVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintbegin {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintbegin {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: ChopConstraintbeginMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trs(mut self, val: ChopConstraintbeginTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstraintbeginXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vex_align(mut self, val: ChopConstraintbeginVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintbeginVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintbeginSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintbeginUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintbeginVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintbegin {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintbegin"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendMethod {
    Proportional = 0,
    Difference = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendRotblend {
    Euler = 0,
    /// Shortest Path (Linear)
    ShortestPathLinear = 1,
    /// Shortest Path (Fast Linear)
    ShortestPathFastLinear = 2,
    /// Euler (Raw Angles)
    EulerRawAngles = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendWritemask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendMask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendVexAlign {
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
pub enum ChopConstraintblendVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintblendUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintblend {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintblend {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3"
    pub fn set_input_sub_network_input_3(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4"
    pub fn set_input_sub_network_input_4(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: "Sub-Network Input #5"
    pub fn set_input_sub_network_input_5(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "Sub-Network Input #5" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: "Sub-Network Input #6"
    pub fn set_input_sub_network_input_6(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "Sub-Network Input #6" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: "Sub-Network Input #7"
    pub fn set_input_sub_network_input_7(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "Sub-Network Input #7" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: "Sub-Network Input #8"
    pub fn set_input_sub_network_input_8(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "Sub-Network Input #8" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: "Sub-Network Input #9"
    pub fn set_input_sub_network_input_9(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "Sub-Network Input #9" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: "Sub-Network Input #10"
    pub fn set_input_sub_network_input_10(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "Sub-Network Input #10" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: "Sub-Network Input #11"
    pub fn set_input_sub_network_input_11(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "Sub-Network Input #11" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: "Sub-Network Input #13"
    pub fn set_input_sub_network_input_13(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "Sub-Network Input #13" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: "Sub-Network Input #13"
    pub fn set_input_sub_network_input_13_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "Sub-Network Input #13" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_13_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: "Sub-Network Input #14"
    pub fn set_input_sub_network_input_14(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "Sub-Network Input #14" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_14_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: "Sub-Network Input #15"
    pub fn set_input_sub_network_input_15(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "Sub-Network Input #15" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_15_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: "Sub-Network Input #16"
    pub fn set_input_sub_network_input_16(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "Sub-Network Input #16" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_16_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(15, (target.get_id(), output_index));
        self
    }

    /// Connects to input 16: "Sub-Network Input #17"
    pub fn set_input_sub_network_input_17(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(16, (target.get_id(), 0));
        self
    }

    /// Connects to input 16: "Sub-Network Input #17" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_17_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(16, (target.get_id(), output_index));
        self
    }

    /// Connects to input 17: "Sub-Network Input #18"
    pub fn set_input_sub_network_input_18(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(17, (target.get_id(), 0));
        self
    }

    /// Connects to input 17: "Sub-Network Input #18" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_18_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(17, (target.get_id(), output_index));
        self
    }

    /// Connects to input 18: "Sub-Network Input #19"
    pub fn set_input_sub_network_input_19(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(18, (target.get_id(), 0));
        self
    }

    /// Connects to input 18: "Sub-Network Input #19" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_19_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(18, (target.get_id(), output_index));
        self
    }

    /// Connects to input 19: "Sub-Network Input #20"
    pub fn set_input_sub_network_input_20(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(19, (target.get_id(), 0));
        self
    }

    /// Connects to input 19: "Sub-Network Input #20" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_20_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(19, (target.get_id(), output_index));
        self
    }

    /// Connects to input 20: "Sub-Network Input #21"
    pub fn set_input_sub_network_input_21(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(20, (target.get_id(), 0));
        self
    }

    /// Connects to input 20: "Sub-Network Input #21" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_21_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(20, (target.get_id(), output_index));
        self
    }

    /// Connects to input 21: "Sub-Network Input #22"
    pub fn set_input_sub_network_input_22(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(21, (target.get_id(), 0));
        self
    }

    /// Connects to input 21: "Sub-Network Input #22" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_22_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(21, (target.get_id(), output_index));
        self
    }

    /// Connects to input 22: "Sub-Network Input #23"
    pub fn set_input_sub_network_input_23(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(22, (target.get_id(), 0));
        self
    }

    /// Connects to input 22: "Sub-Network Input #23" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_23_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(22, (target.get_id(), output_index));
        self
    }

    /// Connects to input 23: "Sub-Network Input #24"
    pub fn set_input_sub_network_input_24(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(23, (target.get_id(), 0));
        self
    }

    /// Connects to input 23: "Sub-Network Input #24" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_24_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(23, (target.get_id(), output_index));
        self
    }

    /// Connects to input 24: "Sub-Network Input #25"
    pub fn set_input_sub_network_input_25(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(24, (target.get_id(), 0));
        self
    }

    /// Connects to input 24: "Sub-Network Input #25" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_25_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(24, (target.get_id(), output_index));
        self
    }

    /// Connects to input 25: "Sub-Network Input #26"
    pub fn set_input_sub_network_input_26(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(25, (target.get_id(), 0));
        self
    }

    /// Connects to input 25: "Sub-Network Input #26" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_26_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(25, (target.get_id(), output_index));
        self
    }

    /// Connects to input 26: "Sub-Network Input #27"
    pub fn set_input_sub_network_input_27(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(26, (target.get_id(), 0));
        self
    }

    /// Connects to input 26: "Sub-Network Input #27" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_27_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(26, (target.get_id(), output_index));
        self
    }

    /// Connects to input 27: "Sub-Network Input #28"
    pub fn set_input_sub_network_input_28(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(27, (target.get_id(), 0));
        self
    }

    /// Connects to input 27: "Sub-Network Input #28" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_28_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(27, (target.get_id(), output_index));
        self
    }

    /// Connects to input 28: "Sub-Network Input #29"
    pub fn set_input_sub_network_input_29(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(28, (target.get_id(), 0));
        self
    }

    /// Connects to input 28: "Sub-Network Input #29" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_29_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(28, (target.get_id(), output_index));
        self
    }

    /// Connects to input 29: "Sub-Network Input #30"
    pub fn set_input_sub_network_input_30(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(29, (target.get_id(), 0));
        self
    }

    /// Connects to input 29: "Sub-Network Input #30" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_30_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(29, (target.get_id(), output_index));
        self
    }

    /// Connects to input 30: "Sub-Network Input #31"
    pub fn set_input_sub_network_input_31(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(30, (target.get_id(), 0));
        self
    }

    /// Connects to input 30: "Sub-Network Input #31" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_31_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(30, (target.get_id(), output_index));
        self
    }

    /// Connects to input 31: "Sub-Network Input #32"
    pub fn set_input_sub_network_input_32(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(31, (target.get_id(), 0));
        self
    }

    /// Connects to input 31: "Sub-Network Input #32" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_32_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(31, (target.get_id(), output_index));
        self
    }

    /// Connects to input 32: "Sub-Network Input #33"
    pub fn set_input_sub_network_input_33(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(32, (target.get_id(), 0));
        self
    }

    /// Connects to input 32: "Sub-Network Input #33" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_33_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(32, (target.get_id(), output_index));
        self
    }

    /// Connects to input 33: "Sub-Network Input #34"
    pub fn set_input_sub_network_input_34(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(33, (target.get_id(), 0));
        self
    }

    /// Connects to input 33: "Sub-Network Input #34" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_34_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(33, (target.get_id(), output_index));
        self
    }

    /// Connects to input 34: "Sub-Network Input #35"
    pub fn set_input_sub_network_input_35(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(34, (target.get_id(), 0));
        self
    }

    /// Connects to input 34: "Sub-Network Input #35" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_35_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(34, (target.get_id(), output_index));
        self
    }

    /// Connects to input 35: "Sub-Network Input #36"
    pub fn set_input_sub_network_input_36(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(35, (target.get_id(), 0));
        self
    }

    /// Connects to input 35: "Sub-Network Input #36" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_36_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(35, (target.get_id(), output_index));
        self
    }

    /// Connects to input 36: "Sub-Network Input #37"
    pub fn set_input_sub_network_input_37(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(36, (target.get_id(), 0));
        self
    }

    /// Connects to input 36: "Sub-Network Input #37" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_37_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(36, (target.get_id(), output_index));
        self
    }

    /// Connects to input 37: "Sub-Network Input #38"
    pub fn set_input_sub_network_input_38(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(37, (target.get_id(), 0));
        self
    }

    /// Connects to input 37: "Sub-Network Input #38" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_38_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(37, (target.get_id(), output_index));
        self
    }

    /// Connects to input 38: "Sub-Network Input #39"
    pub fn set_input_sub_network_input_39(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(38, (target.get_id(), 0));
        self
    }

    /// Connects to input 38: "Sub-Network Input #39" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_39_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(38, (target.get_id(), output_index));
        self
    }

    /// Connects to input 39: "Sub-Network Input #40"
    pub fn set_input_sub_network_input_40(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(39, (target.get_id(), 0));
        self
    }

    /// Connects to input 39: "Sub-Network Input #40" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_40_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(39, (target.get_id(), output_index));
        self
    }

    /// Connects to input 40: "Sub-Network Input #41"
    pub fn set_input_sub_network_input_41(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(40, (target.get_id(), 0));
        self
    }

    /// Connects to input 40: "Sub-Network Input #41" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_41_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(40, (target.get_id(), output_index));
        self
    }

    /// Connects to input 41: "Sub-Network Input #42"
    pub fn set_input_sub_network_input_42(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(41, (target.get_id(), 0));
        self
    }

    /// Connects to input 41: "Sub-Network Input #42" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_42_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(41, (target.get_id(), output_index));
        self
    }

    /// Connects to input 42: "Sub-Network Input #43"
    pub fn set_input_sub_network_input_43(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(42, (target.get_id(), 0));
        self
    }

    /// Connects to input 42: "Sub-Network Input #43" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_43_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(42, (target.get_id(), output_index));
        self
    }

    /// Connects to input 43: "Sub-Network Input #44"
    pub fn set_input_sub_network_input_44(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(43, (target.get_id(), 0));
        self
    }

    /// Connects to input 43: "Sub-Network Input #44" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_44_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(43, (target.get_id(), output_index));
        self
    }

    /// Connects to input 44: "Sub-Network Input #45"
    pub fn set_input_sub_network_input_45(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(44, (target.get_id(), 0));
        self
    }

    /// Connects to input 44: "Sub-Network Input #45" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_45_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(44, (target.get_id(), output_index));
        self
    }

    /// Connects to input 45: "Sub-Network Input #46"
    pub fn set_input_sub_network_input_46(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(45, (target.get_id(), 0));
        self
    }

    /// Connects to input 45: "Sub-Network Input #46" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_46_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(45, (target.get_id(), output_index));
        self
    }

    /// Connects to input 46: "Sub-Network Input #47"
    pub fn set_input_sub_network_input_47(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(46, (target.get_id(), 0));
        self
    }

    /// Connects to input 46: "Sub-Network Input #47" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_47_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(46, (target.get_id(), output_index));
        self
    }

    /// Connects to input 47: "Sub-Network Input #48"
    pub fn set_input_sub_network_input_48(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(47, (target.get_id(), 0));
        self
    }

    /// Connects to input 47: "Sub-Network Input #48" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_48_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(47, (target.get_id(), output_index));
        self
    }

    /// Connects to input 48: "Sub-Network Input #49"
    pub fn set_input_sub_network_input_49(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(48, (target.get_id(), 0));
        self
    }

    /// Connects to input 48: "Sub-Network Input #49" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_49_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(48, (target.get_id(), output_index));
        self
    }

    /// Connects to input 49: "Sub-Network Input #50"
    pub fn set_input_sub_network_input_50(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(49, (target.get_id(), 0));
        self
    }

    /// Connects to input 49: "Sub-Network Input #50" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_50_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(49, (target.get_id(), output_index));
        self
    }

    /// Connects to input 50: "Sub-Network Input #51"
    pub fn set_input_sub_network_input_51(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(50, (target.get_id(), 0));
        self
    }

    /// Connects to input 50: "Sub-Network Input #51" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_51_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(50, (target.get_id(), output_index));
        self
    }

    /// Connects to input 51: "Sub-Network Input #52"
    pub fn set_input_sub_network_input_52(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(51, (target.get_id(), 0));
        self
    }

    /// Connects to input 51: "Sub-Network Input #52" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_52_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(51, (target.get_id(), output_index));
        self
    }

    /// Connects to input 52: "Sub-Network Input #53"
    pub fn set_input_sub_network_input_53(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(52, (target.get_id(), 0));
        self
    }

    /// Connects to input 52: "Sub-Network Input #53" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_53_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(52, (target.get_id(), output_index));
        self
    }

    /// Connects to input 53: "Sub-Network Input #54"
    pub fn set_input_sub_network_input_54(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(53, (target.get_id(), 0));
        self
    }

    /// Connects to input 53: "Sub-Network Input #54" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_54_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(53, (target.get_id(), output_index));
        self
    }

    /// Connects to input 54: "Sub-Network Input #55"
    pub fn set_input_sub_network_input_55(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(54, (target.get_id(), 0));
        self
    }

    /// Connects to input 54: "Sub-Network Input #55" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_55_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(54, (target.get_id(), output_index));
        self
    }

    /// Connects to input 55: "Sub-Network Input #56"
    pub fn set_input_sub_network_input_56(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(55, (target.get_id(), 0));
        self
    }

    /// Connects to input 55: "Sub-Network Input #56" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_56_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(55, (target.get_id(), output_index));
        self
    }

    /// Connects to input 56: "Sub-Network Input #57"
    pub fn set_input_sub_network_input_57(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(56, (target.get_id(), 0));
        self
    }

    /// Connects to input 56: "Sub-Network Input #57" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_57_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(56, (target.get_id(), output_index));
        self
    }

    /// Connects to input 57: "Sub-Network Input #58"
    pub fn set_input_sub_network_input_58(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(57, (target.get_id(), 0));
        self
    }

    /// Connects to input 57: "Sub-Network Input #58" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_58_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(57, (target.get_id(), output_index));
        self
    }

    /// Connects to input 58: "Sub-Network Input #59"
    pub fn set_input_sub_network_input_59(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(58, (target.get_id(), 0));
        self
    }

    /// Connects to input 58: "Sub-Network Input #59" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_59_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(58, (target.get_id(), output_index));
        self
    }

    /// Connects to input 59: "Sub-Network Input #60"
    pub fn set_input_sub_network_input_60(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(59, (target.get_id(), 0));
        self
    }

    /// Connects to input 59: "Sub-Network Input #60" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_60_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(59, (target.get_id(), output_index));
        self
    }

    /// Connects to input 60: "Sub-Network Input #61"
    pub fn set_input_sub_network_input_61(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(60, (target.get_id(), 0));
        self
    }

    /// Connects to input 60: "Sub-Network Input #61" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_61_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(60, (target.get_id(), output_index));
        self
    }

    /// Connects to input 61: "Sub-Network Input #62"
    pub fn set_input_sub_network_input_62(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(61, (target.get_id(), 0));
        self
    }

    /// Connects to input 61: "Sub-Network Input #62" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_62_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(61, (target.get_id(), output_index));
        self
    }

    /// Connects to input 62: "Sub-Network Input #63"
    pub fn set_input_sub_network_input_63(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(62, (target.get_id(), 0));
        self
    }

    /// Connects to input 62: "Sub-Network Input #63" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_63_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(62, (target.get_id(), output_index));
        self
    }

    /// Connects to input 63: "Sub-Network Input #64"
    pub fn set_input_sub_network_input_64(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(63, (target.get_id(), 0));
        self
    }

    /// Connects to input 63: "Sub-Network Input #64" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_64_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(63, (target.get_id(), output_index));
        self
    }

    /// Connects to input 64: "Sub-Network Input #65"
    pub fn set_input_sub_network_input_65(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(64, (target.get_id(), 0));
        self
    }

    /// Connects to input 64: "Sub-Network Input #65" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_65_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(64, (target.get_id(), output_index));
        self
    }

    /// Connects to input 65: "Sub-Network Input #66"
    pub fn set_input_sub_network_input_66(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(65, (target.get_id(), 0));
        self
    }

    /// Connects to input 65: "Sub-Network Input #66" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_66_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(65, (target.get_id(), output_index));
        self
    }

    /// Connects to input 66: "Sub-Network Input #67"
    pub fn set_input_sub_network_input_67(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(66, (target.get_id(), 0));
        self
    }

    /// Connects to input 66: "Sub-Network Input #67" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_67_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(66, (target.get_id(), output_index));
        self
    }

    /// Connects to input 67: "Sub-Network Input #68"
    pub fn set_input_sub_network_input_68(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(67, (target.get_id(), 0));
        self
    }

    /// Connects to input 67: "Sub-Network Input #68" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_68_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(67, (target.get_id(), output_index));
        self
    }

    /// Connects to input 68: "Sub-Network Input #69"
    pub fn set_input_sub_network_input_69(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(68, (target.get_id(), 0));
        self
    }

    /// Connects to input 68: "Sub-Network Input #69" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_69_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(68, (target.get_id(), output_index));
        self
    }

    /// Connects to input 69: "Sub-Network Input #70"
    pub fn set_input_sub_network_input_70(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(69, (target.get_id(), 0));
        self
    }

    /// Connects to input 69: "Sub-Network Input #70" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_70_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(69, (target.get_id(), output_index));
        self
    }

    /// Connects to input 70: "Sub-Network Input #71"
    pub fn set_input_sub_network_input_71(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(70, (target.get_id(), 0));
        self
    }

    /// Connects to input 70: "Sub-Network Input #71" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_71_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(70, (target.get_id(), output_index));
        self
    }

    /// Connects to input 71: "Sub-Network Input #72"
    pub fn set_input_sub_network_input_72(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(71, (target.get_id(), 0));
        self
    }

    /// Connects to input 71: "Sub-Network Input #72" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_72_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(71, (target.get_id(), output_index));
        self
    }

    /// Connects to input 72: "Sub-Network Input #73"
    pub fn set_input_sub_network_input_73(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(72, (target.get_id(), 0));
        self
    }

    /// Connects to input 72: "Sub-Network Input #73" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_73_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(72, (target.get_id(), output_index));
        self
    }

    /// Connects to input 73: "Sub-Network Input #74"
    pub fn set_input_sub_network_input_74(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(73, (target.get_id(), 0));
        self
    }

    /// Connects to input 73: "Sub-Network Input #74" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_74_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(73, (target.get_id(), output_index));
        self
    }

    /// Connects to input 74: "Sub-Network Input #75"
    pub fn set_input_sub_network_input_75(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(74, (target.get_id(), 0));
        self
    }

    /// Connects to input 74: "Sub-Network Input #75" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_75_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(74, (target.get_id(), output_index));
        self
    }

    /// Connects to input 75: "Sub-Network Input #76"
    pub fn set_input_sub_network_input_76(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(75, (target.get_id(), 0));
        self
    }

    /// Connects to input 75: "Sub-Network Input #76" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_76_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(75, (target.get_id(), output_index));
        self
    }

    /// Connects to input 76: "Sub-Network Input #77"
    pub fn set_input_sub_network_input_77(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(76, (target.get_id(), 0));
        self
    }

    /// Connects to input 76: "Sub-Network Input #77" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_77_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(76, (target.get_id(), output_index));
        self
    }

    /// Connects to input 77: "Sub-Network Input #78"
    pub fn set_input_sub_network_input_78(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(77, (target.get_id(), 0));
        self
    }

    /// Connects to input 77: "Sub-Network Input #78" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_78_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(77, (target.get_id(), output_index));
        self
    }

    /// Connects to input 78: "Sub-Network Input #79"
    pub fn set_input_sub_network_input_79(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(78, (target.get_id(), 0));
        self
    }

    /// Connects to input 78: "Sub-Network Input #79" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_79_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(78, (target.get_id(), output_index));
        self
    }

    /// Connects to input 79: "Sub-Network Input #80"
    pub fn set_input_sub_network_input_80(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(79, (target.get_id(), 0));
        self
    }

    /// Connects to input 79: "Sub-Network Input #80" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_80_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(79, (target.get_id(), output_index));
        self
    }

    /// Connects to input 80: "Sub-Network Input #81"
    pub fn set_input_sub_network_input_81(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(80, (target.get_id(), 0));
        self
    }

    /// Connects to input 80: "Sub-Network Input #81" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_81_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(80, (target.get_id(), output_index));
        self
    }

    /// Connects to input 81: "Sub-Network Input #82"
    pub fn set_input_sub_network_input_82(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(81, (target.get_id(), 0));
        self
    }

    /// Connects to input 81: "Sub-Network Input #82" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_82_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(81, (target.get_id(), output_index));
        self
    }

    /// Connects to input 82: "Sub-Network Input #83"
    pub fn set_input_sub_network_input_83(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(82, (target.get_id(), 0));
        self
    }

    /// Connects to input 82: "Sub-Network Input #83" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_83_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(82, (target.get_id(), output_index));
        self
    }

    /// Connects to input 83: "Sub-Network Input #84"
    pub fn set_input_sub_network_input_84(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(83, (target.get_id(), 0));
        self
    }

    /// Connects to input 83: "Sub-Network Input #84" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_84_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(83, (target.get_id(), output_index));
        self
    }

    /// Connects to input 84: "Sub-Network Input #85"
    pub fn set_input_sub_network_input_85(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(84, (target.get_id(), 0));
        self
    }

    /// Connects to input 84: "Sub-Network Input #85" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_85_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(84, (target.get_id(), output_index));
        self
    }

    /// Connects to input 85: "Sub-Network Input #86"
    pub fn set_input_sub_network_input_86(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(85, (target.get_id(), 0));
        self
    }

    /// Connects to input 85: "Sub-Network Input #86" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_86_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(85, (target.get_id(), output_index));
        self
    }

    /// Connects to input 86: "Sub-Network Input #87"
    pub fn set_input_sub_network_input_87(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(86, (target.get_id(), 0));
        self
    }

    /// Connects to input 86: "Sub-Network Input #87" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_87_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(86, (target.get_id(), output_index));
        self
    }

    /// Connects to input 87: "Sub-Network Input #88"
    pub fn set_input_sub_network_input_88(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(87, (target.get_id(), 0));
        self
    }

    /// Connects to input 87: "Sub-Network Input #88" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_88_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(87, (target.get_id(), output_index));
        self
    }

    /// Connects to input 88: "Sub-Network Input #89"
    pub fn set_input_sub_network_input_89(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(88, (target.get_id(), 0));
        self
    }

    /// Connects to input 88: "Sub-Network Input #89" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_89_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(88, (target.get_id(), output_index));
        self
    }

    /// Connects to input 89: "Sub-Network Input #90"
    pub fn set_input_sub_network_input_90(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(89, (target.get_id(), 0));
        self
    }

    /// Connects to input 89: "Sub-Network Input #90" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_90_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(89, (target.get_id(), output_index));
        self
    }

    /// Connects to input 90: "Sub-Network Input #91"
    pub fn set_input_sub_network_input_91(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(90, (target.get_id(), 0));
        self
    }

    /// Connects to input 90: "Sub-Network Input #91" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_91_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(90, (target.get_id(), output_index));
        self
    }

    /// Connects to input 91: "Sub-Network Input #92"
    pub fn set_input_sub_network_input_92(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(91, (target.get_id(), 0));
        self
    }

    /// Connects to input 91: "Sub-Network Input #92" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_92_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(91, (target.get_id(), output_index));
        self
    }

    /// Connects to input 92: "Sub-Network Input #93"
    pub fn set_input_sub_network_input_93(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(92, (target.get_id(), 0));
        self
    }

    /// Connects to input 92: "Sub-Network Input #93" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_93_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(92, (target.get_id(), output_index));
        self
    }

    /// Connects to input 93: "Sub-Network Input #94"
    pub fn set_input_sub_network_input_94(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(93, (target.get_id(), 0));
        self
    }

    /// Connects to input 93: "Sub-Network Input #94" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_94_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(93, (target.get_id(), output_index));
        self
    }

    /// Connects to input 94: "Sub-Network Input #95"
    pub fn set_input_sub_network_input_95(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(94, (target.get_id(), 0));
        self
    }

    /// Connects to input 94: "Sub-Network Input #95" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_95_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(94, (target.get_id(), output_index));
        self
    }

    /// Connects to input 95: "Sub-Network Input #96"
    pub fn set_input_sub_network_input_96(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(95, (target.get_id(), 0));
        self
    }

    /// Connects to input 95: "Sub-Network Input #96" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_96_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(95, (target.get_id(), output_index));
        self
    }

    /// Connects to input 96: "Sub-Network Input #97"
    pub fn set_input_sub_network_input_97(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(96, (target.get_id(), 0));
        self
    }

    /// Connects to input 96: "Sub-Network Input #97" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_97_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(96, (target.get_id(), output_index));
        self
    }

    /// Connects to input 97: "Sub-Network Input #98"
    pub fn set_input_sub_network_input_98(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(97, (target.get_id(), 0));
        self
    }

    /// Connects to input 97: "Sub-Network Input #98" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_98_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(97, (target.get_id(), output_index));
        self
    }

    /// Connects to input 98: "Sub-Network Input #99"
    pub fn set_input_sub_network_input_99(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(98, (target.get_id(), 0));
        self
    }

    /// Connects to input 98: "Sub-Network Input #99" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_99_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(98, (target.get_id(), output_index));
        self
    }

    /// Connects to input 99: "Sub-Network Input #100"
    pub fn set_input_sub_network_input_100(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(99, (target.get_id(), 0));
        self
    }

    /// Connects to input 99: "Sub-Network Input #100" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_100_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(99, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blend_inst(mut self, index1: usize, val: f32) -> Self {
        self.base.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("blend{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_method(mut self, val: ChopConstraintblendMethod) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotblend(mut self, val: ChopConstraintblendRotblend) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rotblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_writemask(mut self, val: ChopConstraintblendWritemask) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mask_inst(mut self, index1: usize, val: ChopConstraintblendMask) -> Self {
        self.base.params.insert(
            format!("mask{}", index1),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mask_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.base.params.insert(
            format!("mask{}", index1),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintblendVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintblendVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintblendVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintblendSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintblendUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintblend {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintblend"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintexportVexAlign {
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
pub enum ChopConstraintexportVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintexportSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintexportUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintexportVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintexport {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintexport {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vex_align(mut self, val: ChopConstraintexportVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintexportVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintexportSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintexportUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintexportVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_constraints_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_constraints_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_constraints_on(mut self, val: bool) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_constraints_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "constraints_on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintexport {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintexport"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetlocalspaceMode {
    UseObjectParmeterTranform = 0,
    FetchNodeParameters = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetlocalspaceTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetlocalspaceXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetlocalspaceVexAlign {
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
pub enum ChopConstraintgetlocalspaceVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetlocalspaceVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetlocalspaceSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetlocalspaceUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintgetlocalspace {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintgetlocalspace {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trs(mut self, val: ChopConstraintgetlocalspaceTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstraintgetlocalspaceXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: ChopConstraintgetlocalspaceMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintgetlocalspaceVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintgetlocalspaceVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintgetlocalspaceVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintgetlocalspaceSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintgetlocalspaceUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintgetlocalspace {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintgetlocalspace"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetparentspaceTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetparentspaceXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetparentspaceParentBone {
    Root = 0,
    Tip = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetparentspaceVexAlign {
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
pub enum ChopConstraintgetparentspaceVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetparentspaceSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetparentspaceUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetparentspaceVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintgetparentspace {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintgetparentspace {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trs(mut self, val: ChopConstraintgetparentspaceTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstraintgetparentspaceXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parent_bone(mut self, val: ChopConstraintgetparentspaceParentBone) -> Self {
        self.base.params.insert(
            "parent_bone".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parent_bone_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "parent_bone".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintgetparentspaceVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintgetparentspaceVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintgetparentspaceSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintgetparentspaceUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintgetparentspaceVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintgetparentspace {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintgetparentspace"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetworldspaceTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetworldspaceXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetworldspaceVexAlign {
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
pub enum ChopConstraintgetworldspaceVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetworldspaceVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetworldspaceSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintgetworldspaceUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintgetworldspace {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintgetworldspace {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trs(mut self, val: ChopConstraintgetworldspaceTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstraintgetworldspaceXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vex_align(mut self, val: ChopConstraintgetworldspaceVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintgetworldspaceVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintgetworldspaceVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintgetworldspaceSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintgetworldspaceUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintgetworldspace {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintgetworldspace"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatLookataxis {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// Z-
    ZMinus = 2,
    /// X+
    XPlus = 3,
    /// Y+
    YPlus = 4,
    /// Z+
    ZPlus = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatLookupaxisx {
    /// Y-
    YMinus = 0,
    /// Z-
    ZMinus = 1,
    /// Y+
    YPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatLookupaxisy {
    /// X-
    XMinus = 0,
    /// Z-
    ZMinus = 1,
    /// X+
    XPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatLookupaxisz {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// X+
    XPlus = 2,
    /// Y+
    YPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatMode {
    UsePosition = 0,
    UseVector = 1,
    UseQuaternion = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatVexAlign {
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
pub enum ChopConstraintlookatVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintlookatUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintlookat {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintlookat {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Lookat"
    pub fn set_input_lookat(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Lookat" and specifies the output index of the target node.
    pub fn set_input_lookat_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Up"
    pub fn set_input_up(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Up" and specifies the output index of the target node.
    pub fn set_input_up_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Twist"
    pub fn set_input_twist(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Twist" and specifies the output index of the target node.
    pub fn set_input_twist_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_twist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_twist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "twist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_lookat(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "lookat".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_lookat_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookat".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uppos(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "uppos".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_uppos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uppos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvec(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "upvec".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvec_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upvec".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_lookataxis(mut self, val: ChopConstraintlookatLookataxis) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookataxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisx(mut self, val: ChopConstraintlookatLookupaxisx) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisy(mut self, val: ChopConstraintlookatLookupaxisy) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisz(mut self, val: ChopConstraintlookatLookupaxisz) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: ChopConstraintlookatMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintlookatVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintlookatVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintlookatVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintlookatSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintlookatUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintlookat {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintlookat"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectVexAlign {
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
pub enum ChopConstraintobjectVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintobject {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintobject {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remove_time_dependent(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__remove_time_dependent__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_remove_time_dependent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__remove_time_dependent__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trs(mut self, val: ChopConstraintobjectTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstraintobjectXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vex_align(mut self, val: ChopConstraintobjectVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintobjectVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintobjectVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintobjectSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintobjectUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ref_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ref_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ref_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ref_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintobject {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintobject"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetMask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetVexAlign {
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
pub enum ChopConstraintobjectoffsetVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectoffsetUnits2 {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintobjectoffset {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintobjectoffset {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Bind Pose"
    pub fn set_input_bind_pose(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Bind Pose" and specifies the output index of the target node.
    pub fn set_input_bind_pose_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_udpate(mut self) -> Self {
        self.base
            .params
            .insert("udpate".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_update(mut self) -> Self {
        self.base
            .params
            .insert("update".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_clear(mut self) -> Self {
        self.base
            .params
            .insert("clear".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_trs(mut self, val: ChopConstraintobjectoffsetTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstraintobjectoffsetXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mask(mut self, val: ChopConstraintobjectoffsetMask) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintobjectoffsetVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintobjectoffsetVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintobjectoffsetVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintobjectoffsetSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintobjectoffsetUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units2(mut self, val: ChopConstraintobjectoffsetUnits2) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_ref_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "ref_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_ref_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "ref_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_opmenu(mut self, val: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintobjectoffset {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintobjectoffset"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ChopConstraintobjectoffsetInnerExt {
    fn bindpose_after(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bindpose_before(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn input1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn invert(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn multiply(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ChopConstraintobjectoffsetInnerExt
    for crate::core::graph::InnerGraph<'a, ChopConstraintobjectoffset>
{
    fn bindpose_after(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bindpose_after")
    }
    fn bindpose_before(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bindpose_before")
    }
    fn input1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("input1")
    }
    fn invert(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("invert")
    }
    fn multiply(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("multiply")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectpretransformTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectpretransformXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectpretransformVexAlign {
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
pub enum ChopConstraintobjectpretransformVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectpretransformVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectpretransformSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintobjectpretransformUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintobjectpretransform {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintobjectpretransform {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remove_time_dependent(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__remove_time_dependent__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_remove_time_dependent_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__remove_time_dependent__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trs(mut self, val: ChopConstraintobjectpretransformTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstraintobjectpretransformXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vex_align(mut self, val: ChopConstraintobjectpretransformVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintobjectpretransformVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(
        mut self,
        val: ChopConstraintobjectpretransformVexNumThreads,
    ) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintobjectpretransformSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintobjectpretransformUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_obj_path(mut self, val: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_obj_path_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "obj_path".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintobjectpretransform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintobjectpretransform"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetRotblend {
    Euler = 0,
    ShortestPath = 1,
    /// Euler (Raw Angles)
    EulerRawAngles = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetWritemask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetMask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetVexAlign {
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
pub enum ChopConstraintoffsetVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetUnits2 {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintoffset {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintoffset {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Bind Pose"
    pub fn set_input_bind_pose(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Bind Pose" and specifies the output index of the target node.
    pub fn set_input_bind_pose_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_update(mut self) -> Self {
        self.base
            .params
            .insert("update".to_string(), crate::core::types::ParamValue::Button);
        self
    }
    pub fn trigger_clear(mut self) -> Self {
        self.base
            .params
            .insert("clear".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_rotblend(mut self, val: ChopConstraintoffsetRotblend) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rotblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_writemask(mut self, val: ChopConstraintoffsetWritemask) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mask(mut self, val: ChopConstraintoffsetMask) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintoffsetVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintoffsetVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintoffsetVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintoffsetSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintoffsetUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units2(mut self, val: ChopConstraintoffsetUnits2) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_opmenu(mut self, val: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintoffset {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintoffset"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ChopConstraintoffsetInnerExt {
    fn bindpose_after(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn bindpose_before(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn constraintoffsetblend1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn constraintsimpleblend1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn invert(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn multiply(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn multiply1(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ChopConstraintoffsetInnerExt for crate::core::graph::InnerGraph<'a, ChopConstraintoffset> {
    fn bindpose_after(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bindpose_after")
    }
    fn bindpose_before(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("bindpose_before")
    }
    fn constraintoffsetblend1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("constraintoffsetblend1")
    }
    fn constraintsimpleblend1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("constraintsimpleblend1")
    }
    fn invert(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("invert")
    }
    fn multiply(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("multiply")
    }
    fn multiply1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("multiply1")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetblendRotblend {
    Euler = 0,
    ShortestPath = 1,
    /// Euler (Raw Angles)
    EulerRawAngles = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetblendWritemask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetblendVexAlign {
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
pub enum ChopConstraintoffsetblendVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetblendVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetblendSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintoffsetblendUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintoffsetblend {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintoffsetblend {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "InvertedBindposeAfter"
    pub fn set_input_invertedbindposeafter(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "InvertedBindposeAfter" and specifies the output index of the target node.
    pub fn set_input_invertedbindposeafter_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Bindpose"
    pub fn set_input_bindpose(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Bindpose" and specifies the output index of the target node.
    pub fn set_input_bindpose_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "BindposeBefore"
    pub fn set_input_bindposebefore(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "BindposeBefore" and specifies the output index of the target node.
    pub fn set_input_bindposebefore_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotblend(mut self, val: ChopConstraintoffsetblendRotblend) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rotblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_writemask(mut self, val: ChopConstraintoffsetblendWritemask) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintoffsetblendVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintoffsetblendVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintoffsetblendVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintoffsetblendSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintoffsetblendUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintoffsetblend {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintoffsetblend"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentVexAlign {
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
pub enum ChopConstraintparentVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintparent {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintparent {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Current Parent"
    pub fn set_input_current_parent(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Current Parent" and specifies the output index of the target node.
    pub fn set_input_current_parent_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "New Parent"
    pub fn set_input_new_parent(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "New Parent" and specifies the output index of the target node.
    pub fn set_input_new_parent_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vex_align(mut self, val: ChopConstraintparentVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintparentVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintparentVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintparentSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintparentUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintparent {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintparent"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentxWritemask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentxVexAlign {
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
pub enum ChopConstraintparentxVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentxVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentxSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentxUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintparentxUnits2 {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintparentx {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintparentx {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Original Parent"
    pub fn set_input_original_parent(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Original Parent" and specifies the output index of the target node.
    pub fn set_input_original_parent_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "New Parent"
    pub fn set_input_new_parent(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "New Parent" and specifies the output index of the target node.
    pub fn set_input_new_parent_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_writemask(mut self, val: ChopConstraintparentxWritemask) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintparentxVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintparentxVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintparentxVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintparentxSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintparentxUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units2(mut self, val: ChopConstraintparentxUnits2) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_opmenu(mut self, val: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_opmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "opmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintparentx {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintparentx"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
#[allow(clippy::wrong_self_convention, non_snake_case)]
pub trait ChopConstraintparentxInnerExt {
    fn blendmask(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output1(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn output2(&mut self) -> crate::core::graph::ExistingNodeRef;
    fn parent(&mut self) -> crate::core::graph::ExistingNodeRef;
}

#[allow(clippy::wrong_self_convention, non_snake_case)]
impl<'a> ChopConstraintparentxInnerExt
    for crate::core::graph::InnerGraph<'a, ChopConstraintparentx>
{
    fn blendmask(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("blendmask")
    }
    fn output1(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output1")
    }
    fn output2(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("output2")
    }
    fn parent(&mut self) -> crate::core::graph::ExistingNodeRef {
        self.existing_node("parent")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathUparmtype {
    NormalizedDistance = 0,
    NormalizedSpline = 1,
    DistanceFromStart = 2,
    DistanceFromEnd = 3,
    DistancePointAttribute = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathLookatmode {
    None = 0,
    AlongPath = 1,
    DirectionAttributeFromPath = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathLookupmode {
    UpVector = 0,
    UpVectorAttributeFromPath = 1,
    AlongPath = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathLookataxis {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// Z-
    ZMinus = 2,
    /// X+
    XPlus = 3,
    /// Y+
    YPlus = 4,
    /// Z+
    ZPlus = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathLookupaxisx {
    /// Y-
    YMinus = 0,
    /// Z-
    ZMinus = 1,
    /// Y+
    YPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathLookupaxisy {
    /// X-
    XMinus = 0,
    /// Z-
    ZMinus = 1,
    /// X+
    XPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathLookupaxisz {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// X+
    XPlus = 2,
    /// Y+
    YPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathVexAlign {
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
pub enum ChopConstraintpathVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpathUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintpath {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintpath {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Lookat"
    pub fn set_input_lookat(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Lookat" and specifies the output index of the target node.
    pub fn set_input_lookat_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Up"
    pub fn set_input_up(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Up" and specifies the output index of the target node.
    pub fn set_input_up_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Position/Roll"
    pub fn set_input_position_roll(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Position/Roll" and specifies the output index of the target node.
    pub fn set_input_position_roll_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_pos(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pos_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pos".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_tolerance(mut self, val: f32) -> Self {
        self.base.params.insert(
            "tolerance".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_tolerance_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "tolerance".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_uparmtype(mut self, val: ChopConstraintpathUparmtype) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uparmtype_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uparmtype".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatmode(mut self, val: ChopConstraintpathLookatmode) -> Self {
        self.base.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookatmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupmode(mut self, val: ChopConstraintpathLookupmode) -> Self {
        self.base.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookataxis(mut self, val: ChopConstraintpathLookataxis) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookataxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisx(mut self, val: ChopConstraintpathLookupaxisx) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisy(mut self, val: ChopConstraintpathLookupaxisy) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisz(mut self, val: ChopConstraintpathLookupaxisz) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintpathVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintpathVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintpathVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintpathSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintpathUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_distattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "distattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_distattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "distattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dirattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintpath {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintpath"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsMode {
    PointGroup = 0,
    PrimitiveGroup = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsLookatmode {
    None = 0,
    DirectionAttributeFromPoints = 1,
    DirectionVectorFromP0ToP1 = 2,
    /// Normal Vector of P0,P1,P2 Plane
    NormalVectorOfP0P1P2Plane = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsLookupmode {
    UpVector = 0,
    UpVectorAttributeFromPoints = 1,
    DirectionVectorFromP0ToP1 = 2,
    /// Normal Vector of P0,P1,P2 Plane
    NormalVectorOfP0P1P2Plane = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsLookataxis {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// Z-
    ZMinus = 2,
    /// X+
    XPlus = 3,
    /// Y+
    YPlus = 4,
    /// Z+
    ZPlus = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsLookupaxisx {
    /// Y-
    YMinus = 0,
    /// Z-
    ZMinus = 1,
    /// Y+
    YPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsLookupaxisy {
    /// X-
    XMinus = 0,
    /// Z-
    ZMinus = 1,
    /// X+
    XPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsLookupaxisz {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// X+
    XPlus = 2,
    /// Y+
    YPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsVexAlign {
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
pub enum ChopConstraintpointsVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintpointsUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintpoints {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintpoints {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Lookat"
    pub fn set_input_lookat(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Lookat" and specifies the output index of the target node.
    pub fn set_input_lookat_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Up"
    pub fn set_input_up(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Up" and specifies the output index of the target node.
    pub fn set_input_up_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Coordinates"
    pub fn set_input_coordinates(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Coordinates" and specifies the output index of the target node.
    pub fn set_input_coordinates_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_searchdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "searchdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_searchdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "searchdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_weights(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "weights".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_weights_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "weights".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_searchmaxpnt(mut self, val: i32) -> Self {
        self.base.params.insert(
            "searchmaxpnt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_searchmaxpnt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "searchmaxpnt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: ChopConstraintpointsMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatmode(mut self, val: ChopConstraintpointsLookatmode) -> Self {
        self.base.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookatmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupmode(mut self, val: ChopConstraintpointsLookupmode) -> Self {
        self.base.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookataxis(mut self, val: ChopConstraintpointsLookataxis) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookataxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisx(mut self, val: ChopConstraintpointsLookupaxisx) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisy(mut self, val: ChopConstraintpointsLookupaxisy) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisz(mut self, val: ChopConstraintpointsLookupaxisz) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintpointsVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintpointsVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintpointsVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintpointsSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintpointsUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dirattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintpoints {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintpoints"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsequenceRotblend {
    Euler = 0,
    ShortestPath = 1,
    /// Euler (Raw Angles)
    EulerRawAngles = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsequenceWritemask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsequenceVexAlign {
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
pub enum ChopConstraintsequenceVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsequenceVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsequenceSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsequenceUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintsequence {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintsequence {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3"
    pub fn set_input_sub_network_input_3(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Sub-Network Input #3" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_3_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4"
    pub fn set_input_sub_network_input_4(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Sub-Network Input #4" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_4_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    /// Connects to input 4: "Sub-Network Input #5"
    pub fn set_input_sub_network_input_5(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), 0));
        self
    }

    /// Connects to input 4: "Sub-Network Input #5" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_5_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(4, (target.get_id(), output_index));
        self
    }

    /// Connects to input 5: "Sub-Network Input #6"
    pub fn set_input_sub_network_input_6(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), 0));
        self
    }

    /// Connects to input 5: "Sub-Network Input #6" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_6_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(5, (target.get_id(), output_index));
        self
    }

    /// Connects to input 6: "Sub-Network Input #7"
    pub fn set_input_sub_network_input_7(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), 0));
        self
    }

    /// Connects to input 6: "Sub-Network Input #7" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_7_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(6, (target.get_id(), output_index));
        self
    }

    /// Connects to input 7: "Sub-Network Input #8"
    pub fn set_input_sub_network_input_8(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), 0));
        self
    }

    /// Connects to input 7: "Sub-Network Input #8" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_8_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(7, (target.get_id(), output_index));
        self
    }

    /// Connects to input 8: "Sub-Network Input #9"
    pub fn set_input_sub_network_input_9(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), 0));
        self
    }

    /// Connects to input 8: "Sub-Network Input #9" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_9_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(8, (target.get_id(), output_index));
        self
    }

    /// Connects to input 9: "Sub-Network Input #10"
    pub fn set_input_sub_network_input_10(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), 0));
        self
    }

    /// Connects to input 9: "Sub-Network Input #10" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_10_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(9, (target.get_id(), output_index));
        self
    }

    /// Connects to input 10: "Sub-Network Input #11"
    pub fn set_input_sub_network_input_11(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), 0));
        self
    }

    /// Connects to input 10: "Sub-Network Input #11" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_11_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(10, (target.get_id(), output_index));
        self
    }

    /// Connects to input 11: "Sub-Network Input #13"
    pub fn set_input_sub_network_input_13(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), 0));
        self
    }

    /// Connects to input 11: "Sub-Network Input #13" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_13_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(11, (target.get_id(), output_index));
        self
    }

    /// Connects to input 12: "Sub-Network Input #13"
    pub fn set_input_sub_network_input_13_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), 0));
        self
    }

    /// Connects to input 12: "Sub-Network Input #13" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_13_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(12, (target.get_id(), output_index));
        self
    }

    /// Connects to input 13: "Sub-Network Input #14"
    pub fn set_input_sub_network_input_14(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), 0));
        self
    }

    /// Connects to input 13: "Sub-Network Input #14" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_14_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(13, (target.get_id(), output_index));
        self
    }

    /// Connects to input 14: "Sub-Network Input #15"
    pub fn set_input_sub_network_input_15(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), 0));
        self
    }

    /// Connects to input 14: "Sub-Network Input #15" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_15_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(14, (target.get_id(), output_index));
        self
    }

    /// Connects to input 15: "Sub-Network Input #16"
    pub fn set_input_sub_network_input_16(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(15, (target.get_id(), 0));
        self
    }

    /// Connects to input 15: "Sub-Network Input #16" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_16_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(15, (target.get_id(), output_index));
        self
    }

    /// Connects to input 16: "Sub-Network Input #17"
    pub fn set_input_sub_network_input_17(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(16, (target.get_id(), 0));
        self
    }

    /// Connects to input 16: "Sub-Network Input #17" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_17_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(16, (target.get_id(), output_index));
        self
    }

    /// Connects to input 17: "Sub-Network Input #18"
    pub fn set_input_sub_network_input_18(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(17, (target.get_id(), 0));
        self
    }

    /// Connects to input 17: "Sub-Network Input #18" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_18_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(17, (target.get_id(), output_index));
        self
    }

    /// Connects to input 18: "Sub-Network Input #19"
    pub fn set_input_sub_network_input_19(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(18, (target.get_id(), 0));
        self
    }

    /// Connects to input 18: "Sub-Network Input #19" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_19_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(18, (target.get_id(), output_index));
        self
    }

    /// Connects to input 19: "Sub-Network Input #20"
    pub fn set_input_sub_network_input_20(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(19, (target.get_id(), 0));
        self
    }

    /// Connects to input 19: "Sub-Network Input #20" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_20_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(19, (target.get_id(), output_index));
        self
    }

    /// Connects to input 20: "Sub-Network Input #21"
    pub fn set_input_sub_network_input_21(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(20, (target.get_id(), 0));
        self
    }

    /// Connects to input 20: "Sub-Network Input #21" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_21_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(20, (target.get_id(), output_index));
        self
    }

    /// Connects to input 21: "Sub-Network Input #22"
    pub fn set_input_sub_network_input_22(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(21, (target.get_id(), 0));
        self
    }

    /// Connects to input 21: "Sub-Network Input #22" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_22_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(21, (target.get_id(), output_index));
        self
    }

    /// Connects to input 22: "Sub-Network Input #23"
    pub fn set_input_sub_network_input_23(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(22, (target.get_id(), 0));
        self
    }

    /// Connects to input 22: "Sub-Network Input #23" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_23_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(22, (target.get_id(), output_index));
        self
    }

    /// Connects to input 23: "Sub-Network Input #24"
    pub fn set_input_sub_network_input_24(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(23, (target.get_id(), 0));
        self
    }

    /// Connects to input 23: "Sub-Network Input #24" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_24_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(23, (target.get_id(), output_index));
        self
    }

    /// Connects to input 24: "Sub-Network Input #25"
    pub fn set_input_sub_network_input_25(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(24, (target.get_id(), 0));
        self
    }

    /// Connects to input 24: "Sub-Network Input #25" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_25_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(24, (target.get_id(), output_index));
        self
    }

    /// Connects to input 25: "Sub-Network Input #26"
    pub fn set_input_sub_network_input_26(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(25, (target.get_id(), 0));
        self
    }

    /// Connects to input 25: "Sub-Network Input #26" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_26_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(25, (target.get_id(), output_index));
        self
    }

    /// Connects to input 26: "Sub-Network Input #27"
    pub fn set_input_sub_network_input_27(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(26, (target.get_id(), 0));
        self
    }

    /// Connects to input 26: "Sub-Network Input #27" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_27_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(26, (target.get_id(), output_index));
        self
    }

    /// Connects to input 27: "Sub-Network Input #28"
    pub fn set_input_sub_network_input_28(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(27, (target.get_id(), 0));
        self
    }

    /// Connects to input 27: "Sub-Network Input #28" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_28_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(27, (target.get_id(), output_index));
        self
    }

    /// Connects to input 28: "Sub-Network Input #29"
    pub fn set_input_sub_network_input_29(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(28, (target.get_id(), 0));
        self
    }

    /// Connects to input 28: "Sub-Network Input #29" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_29_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(28, (target.get_id(), output_index));
        self
    }

    /// Connects to input 29: "Sub-Network Input #30"
    pub fn set_input_sub_network_input_30(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(29, (target.get_id(), 0));
        self
    }

    /// Connects to input 29: "Sub-Network Input #30" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_30_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(29, (target.get_id(), output_index));
        self
    }

    /// Connects to input 30: "Sub-Network Input #31"
    pub fn set_input_sub_network_input_31(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(30, (target.get_id(), 0));
        self
    }

    /// Connects to input 30: "Sub-Network Input #31" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_31_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(30, (target.get_id(), output_index));
        self
    }

    /// Connects to input 31: "Sub-Network Input #32"
    pub fn set_input_sub_network_input_32(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(31, (target.get_id(), 0));
        self
    }

    /// Connects to input 31: "Sub-Network Input #32" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_32_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(31, (target.get_id(), output_index));
        self
    }

    /// Connects to input 32: "Sub-Network Input #33"
    pub fn set_input_sub_network_input_33(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(32, (target.get_id(), 0));
        self
    }

    /// Connects to input 32: "Sub-Network Input #33" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_33_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(32, (target.get_id(), output_index));
        self
    }

    /// Connects to input 33: "Sub-Network Input #34"
    pub fn set_input_sub_network_input_34(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(33, (target.get_id(), 0));
        self
    }

    /// Connects to input 33: "Sub-Network Input #34" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_34_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(33, (target.get_id(), output_index));
        self
    }

    /// Connects to input 34: "Sub-Network Input #35"
    pub fn set_input_sub_network_input_35(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(34, (target.get_id(), 0));
        self
    }

    /// Connects to input 34: "Sub-Network Input #35" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_35_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(34, (target.get_id(), output_index));
        self
    }

    /// Connects to input 35: "Sub-Network Input #36"
    pub fn set_input_sub_network_input_36(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(35, (target.get_id(), 0));
        self
    }

    /// Connects to input 35: "Sub-Network Input #36" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_36_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(35, (target.get_id(), output_index));
        self
    }

    /// Connects to input 36: "Sub-Network Input #37"
    pub fn set_input_sub_network_input_37(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(36, (target.get_id(), 0));
        self
    }

    /// Connects to input 36: "Sub-Network Input #37" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_37_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(36, (target.get_id(), output_index));
        self
    }

    /// Connects to input 37: "Sub-Network Input #38"
    pub fn set_input_sub_network_input_38(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(37, (target.get_id(), 0));
        self
    }

    /// Connects to input 37: "Sub-Network Input #38" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_38_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(37, (target.get_id(), output_index));
        self
    }

    /// Connects to input 38: "Sub-Network Input #39"
    pub fn set_input_sub_network_input_39(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(38, (target.get_id(), 0));
        self
    }

    /// Connects to input 38: "Sub-Network Input #39" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_39_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(38, (target.get_id(), output_index));
        self
    }

    /// Connects to input 39: "Sub-Network Input #40"
    pub fn set_input_sub_network_input_40(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(39, (target.get_id(), 0));
        self
    }

    /// Connects to input 39: "Sub-Network Input #40" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_40_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(39, (target.get_id(), output_index));
        self
    }

    /// Connects to input 40: "Sub-Network Input #41"
    pub fn set_input_sub_network_input_41(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(40, (target.get_id(), 0));
        self
    }

    /// Connects to input 40: "Sub-Network Input #41" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_41_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(40, (target.get_id(), output_index));
        self
    }

    /// Connects to input 41: "Sub-Network Input #42"
    pub fn set_input_sub_network_input_42(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(41, (target.get_id(), 0));
        self
    }

    /// Connects to input 41: "Sub-Network Input #42" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_42_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(41, (target.get_id(), output_index));
        self
    }

    /// Connects to input 42: "Sub-Network Input #43"
    pub fn set_input_sub_network_input_43(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(42, (target.get_id(), 0));
        self
    }

    /// Connects to input 42: "Sub-Network Input #43" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_43_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(42, (target.get_id(), output_index));
        self
    }

    /// Connects to input 43: "Sub-Network Input #44"
    pub fn set_input_sub_network_input_44(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(43, (target.get_id(), 0));
        self
    }

    /// Connects to input 43: "Sub-Network Input #44" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_44_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(43, (target.get_id(), output_index));
        self
    }

    /// Connects to input 44: "Sub-Network Input #45"
    pub fn set_input_sub_network_input_45(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(44, (target.get_id(), 0));
        self
    }

    /// Connects to input 44: "Sub-Network Input #45" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_45_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(44, (target.get_id(), output_index));
        self
    }

    /// Connects to input 45: "Sub-Network Input #46"
    pub fn set_input_sub_network_input_46(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(45, (target.get_id(), 0));
        self
    }

    /// Connects to input 45: "Sub-Network Input #46" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_46_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(45, (target.get_id(), output_index));
        self
    }

    /// Connects to input 46: "Sub-Network Input #47"
    pub fn set_input_sub_network_input_47(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(46, (target.get_id(), 0));
        self
    }

    /// Connects to input 46: "Sub-Network Input #47" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_47_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(46, (target.get_id(), output_index));
        self
    }

    /// Connects to input 47: "Sub-Network Input #48"
    pub fn set_input_sub_network_input_48(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(47, (target.get_id(), 0));
        self
    }

    /// Connects to input 47: "Sub-Network Input #48" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_48_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(47, (target.get_id(), output_index));
        self
    }

    /// Connects to input 48: "Sub-Network Input #49"
    pub fn set_input_sub_network_input_49(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(48, (target.get_id(), 0));
        self
    }

    /// Connects to input 48: "Sub-Network Input #49" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_49_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(48, (target.get_id(), output_index));
        self
    }

    /// Connects to input 49: "Sub-Network Input #50"
    pub fn set_input_sub_network_input_50(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(49, (target.get_id(), 0));
        self
    }

    /// Connects to input 49: "Sub-Network Input #50" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_50_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(49, (target.get_id(), output_index));
        self
    }

    /// Connects to input 50: "Sub-Network Input #51"
    pub fn set_input_sub_network_input_51(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(50, (target.get_id(), 0));
        self
    }

    /// Connects to input 50: "Sub-Network Input #51" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_51_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(50, (target.get_id(), output_index));
        self
    }

    /// Connects to input 51: "Sub-Network Input #52"
    pub fn set_input_sub_network_input_52(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(51, (target.get_id(), 0));
        self
    }

    /// Connects to input 51: "Sub-Network Input #52" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_52_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(51, (target.get_id(), output_index));
        self
    }

    /// Connects to input 52: "Sub-Network Input #53"
    pub fn set_input_sub_network_input_53(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(52, (target.get_id(), 0));
        self
    }

    /// Connects to input 52: "Sub-Network Input #53" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_53_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(52, (target.get_id(), output_index));
        self
    }

    /// Connects to input 53: "Sub-Network Input #54"
    pub fn set_input_sub_network_input_54(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(53, (target.get_id(), 0));
        self
    }

    /// Connects to input 53: "Sub-Network Input #54" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_54_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(53, (target.get_id(), output_index));
        self
    }

    /// Connects to input 54: "Sub-Network Input #55"
    pub fn set_input_sub_network_input_55(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(54, (target.get_id(), 0));
        self
    }

    /// Connects to input 54: "Sub-Network Input #55" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_55_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(54, (target.get_id(), output_index));
        self
    }

    /// Connects to input 55: "Sub-Network Input #56"
    pub fn set_input_sub_network_input_56(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(55, (target.get_id(), 0));
        self
    }

    /// Connects to input 55: "Sub-Network Input #56" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_56_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(55, (target.get_id(), output_index));
        self
    }

    /// Connects to input 56: "Sub-Network Input #57"
    pub fn set_input_sub_network_input_57(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(56, (target.get_id(), 0));
        self
    }

    /// Connects to input 56: "Sub-Network Input #57" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_57_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(56, (target.get_id(), output_index));
        self
    }

    /// Connects to input 57: "Sub-Network Input #58"
    pub fn set_input_sub_network_input_58(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(57, (target.get_id(), 0));
        self
    }

    /// Connects to input 57: "Sub-Network Input #58" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_58_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(57, (target.get_id(), output_index));
        self
    }

    /// Connects to input 58: "Sub-Network Input #59"
    pub fn set_input_sub_network_input_59(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(58, (target.get_id(), 0));
        self
    }

    /// Connects to input 58: "Sub-Network Input #59" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_59_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(58, (target.get_id(), output_index));
        self
    }

    /// Connects to input 59: "Sub-Network Input #60"
    pub fn set_input_sub_network_input_60(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(59, (target.get_id(), 0));
        self
    }

    /// Connects to input 59: "Sub-Network Input #60" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_60_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(59, (target.get_id(), output_index));
        self
    }

    /// Connects to input 60: "Sub-Network Input #61"
    pub fn set_input_sub_network_input_61(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(60, (target.get_id(), 0));
        self
    }

    /// Connects to input 60: "Sub-Network Input #61" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_61_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(60, (target.get_id(), output_index));
        self
    }

    /// Connects to input 61: "Sub-Network Input #62"
    pub fn set_input_sub_network_input_62(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(61, (target.get_id(), 0));
        self
    }

    /// Connects to input 61: "Sub-Network Input #62" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_62_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(61, (target.get_id(), output_index));
        self
    }

    /// Connects to input 62: "Sub-Network Input #63"
    pub fn set_input_sub_network_input_63(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(62, (target.get_id(), 0));
        self
    }

    /// Connects to input 62: "Sub-Network Input #63" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_63_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(62, (target.get_id(), output_index));
        self
    }

    /// Connects to input 63: "Sub-Network Input #64"
    pub fn set_input_sub_network_input_64(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(63, (target.get_id(), 0));
        self
    }

    /// Connects to input 63: "Sub-Network Input #64" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_64_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(63, (target.get_id(), output_index));
        self
    }

    /// Connects to input 64: "Sub-Network Input #65"
    pub fn set_input_sub_network_input_65(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(64, (target.get_id(), 0));
        self
    }

    /// Connects to input 64: "Sub-Network Input #65" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_65_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(64, (target.get_id(), output_index));
        self
    }

    /// Connects to input 65: "Sub-Network Input #66"
    pub fn set_input_sub_network_input_66(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(65, (target.get_id(), 0));
        self
    }

    /// Connects to input 65: "Sub-Network Input #66" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_66_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(65, (target.get_id(), output_index));
        self
    }

    /// Connects to input 66: "Sub-Network Input #67"
    pub fn set_input_sub_network_input_67(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(66, (target.get_id(), 0));
        self
    }

    /// Connects to input 66: "Sub-Network Input #67" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_67_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(66, (target.get_id(), output_index));
        self
    }

    /// Connects to input 67: "Sub-Network Input #68"
    pub fn set_input_sub_network_input_68(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(67, (target.get_id(), 0));
        self
    }

    /// Connects to input 67: "Sub-Network Input #68" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_68_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(67, (target.get_id(), output_index));
        self
    }

    /// Connects to input 68: "Sub-Network Input #69"
    pub fn set_input_sub_network_input_69(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(68, (target.get_id(), 0));
        self
    }

    /// Connects to input 68: "Sub-Network Input #69" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_69_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(68, (target.get_id(), output_index));
        self
    }

    /// Connects to input 69: "Sub-Network Input #70"
    pub fn set_input_sub_network_input_70(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(69, (target.get_id(), 0));
        self
    }

    /// Connects to input 69: "Sub-Network Input #70" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_70_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(69, (target.get_id(), output_index));
        self
    }

    /// Connects to input 70: "Sub-Network Input #71"
    pub fn set_input_sub_network_input_71(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(70, (target.get_id(), 0));
        self
    }

    /// Connects to input 70: "Sub-Network Input #71" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_71_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(70, (target.get_id(), output_index));
        self
    }

    /// Connects to input 71: "Sub-Network Input #72"
    pub fn set_input_sub_network_input_72(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(71, (target.get_id(), 0));
        self
    }

    /// Connects to input 71: "Sub-Network Input #72" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_72_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(71, (target.get_id(), output_index));
        self
    }

    /// Connects to input 72: "Sub-Network Input #73"
    pub fn set_input_sub_network_input_73(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(72, (target.get_id(), 0));
        self
    }

    /// Connects to input 72: "Sub-Network Input #73" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_73_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(72, (target.get_id(), output_index));
        self
    }

    /// Connects to input 73: "Sub-Network Input #74"
    pub fn set_input_sub_network_input_74(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(73, (target.get_id(), 0));
        self
    }

    /// Connects to input 73: "Sub-Network Input #74" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_74_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(73, (target.get_id(), output_index));
        self
    }

    /// Connects to input 74: "Sub-Network Input #75"
    pub fn set_input_sub_network_input_75(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(74, (target.get_id(), 0));
        self
    }

    /// Connects to input 74: "Sub-Network Input #75" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_75_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(74, (target.get_id(), output_index));
        self
    }

    /// Connects to input 75: "Sub-Network Input #76"
    pub fn set_input_sub_network_input_76(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(75, (target.get_id(), 0));
        self
    }

    /// Connects to input 75: "Sub-Network Input #76" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_76_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(75, (target.get_id(), output_index));
        self
    }

    /// Connects to input 76: "Sub-Network Input #77"
    pub fn set_input_sub_network_input_77(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(76, (target.get_id(), 0));
        self
    }

    /// Connects to input 76: "Sub-Network Input #77" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_77_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(76, (target.get_id(), output_index));
        self
    }

    /// Connects to input 77: "Sub-Network Input #78"
    pub fn set_input_sub_network_input_78(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(77, (target.get_id(), 0));
        self
    }

    /// Connects to input 77: "Sub-Network Input #78" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_78_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(77, (target.get_id(), output_index));
        self
    }

    /// Connects to input 78: "Sub-Network Input #79"
    pub fn set_input_sub_network_input_79(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(78, (target.get_id(), 0));
        self
    }

    /// Connects to input 78: "Sub-Network Input #79" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_79_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(78, (target.get_id(), output_index));
        self
    }

    /// Connects to input 79: "Sub-Network Input #80"
    pub fn set_input_sub_network_input_80(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(79, (target.get_id(), 0));
        self
    }

    /// Connects to input 79: "Sub-Network Input #80" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_80_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(79, (target.get_id(), output_index));
        self
    }

    /// Connects to input 80: "Sub-Network Input #81"
    pub fn set_input_sub_network_input_81(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(80, (target.get_id(), 0));
        self
    }

    /// Connects to input 80: "Sub-Network Input #81" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_81_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(80, (target.get_id(), output_index));
        self
    }

    /// Connects to input 81: "Sub-Network Input #82"
    pub fn set_input_sub_network_input_82(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(81, (target.get_id(), 0));
        self
    }

    /// Connects to input 81: "Sub-Network Input #82" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_82_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(81, (target.get_id(), output_index));
        self
    }

    /// Connects to input 82: "Sub-Network Input #83"
    pub fn set_input_sub_network_input_83(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(82, (target.get_id(), 0));
        self
    }

    /// Connects to input 82: "Sub-Network Input #83" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_83_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(82, (target.get_id(), output_index));
        self
    }

    /// Connects to input 83: "Sub-Network Input #84"
    pub fn set_input_sub_network_input_84(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(83, (target.get_id(), 0));
        self
    }

    /// Connects to input 83: "Sub-Network Input #84" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_84_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(83, (target.get_id(), output_index));
        self
    }

    /// Connects to input 84: "Sub-Network Input #85"
    pub fn set_input_sub_network_input_85(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(84, (target.get_id(), 0));
        self
    }

    /// Connects to input 84: "Sub-Network Input #85" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_85_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(84, (target.get_id(), output_index));
        self
    }

    /// Connects to input 85: "Sub-Network Input #86"
    pub fn set_input_sub_network_input_86(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(85, (target.get_id(), 0));
        self
    }

    /// Connects to input 85: "Sub-Network Input #86" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_86_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(85, (target.get_id(), output_index));
        self
    }

    /// Connects to input 86: "Sub-Network Input #87"
    pub fn set_input_sub_network_input_87(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(86, (target.get_id(), 0));
        self
    }

    /// Connects to input 86: "Sub-Network Input #87" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_87_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(86, (target.get_id(), output_index));
        self
    }

    /// Connects to input 87: "Sub-Network Input #88"
    pub fn set_input_sub_network_input_88(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(87, (target.get_id(), 0));
        self
    }

    /// Connects to input 87: "Sub-Network Input #88" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_88_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(87, (target.get_id(), output_index));
        self
    }

    /// Connects to input 88: "Sub-Network Input #89"
    pub fn set_input_sub_network_input_89(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(88, (target.get_id(), 0));
        self
    }

    /// Connects to input 88: "Sub-Network Input #89" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_89_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(88, (target.get_id(), output_index));
        self
    }

    /// Connects to input 89: "Sub-Network Input #90"
    pub fn set_input_sub_network_input_90(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(89, (target.get_id(), 0));
        self
    }

    /// Connects to input 89: "Sub-Network Input #90" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_90_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(89, (target.get_id(), output_index));
        self
    }

    /// Connects to input 90: "Sub-Network Input #91"
    pub fn set_input_sub_network_input_91(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(90, (target.get_id(), 0));
        self
    }

    /// Connects to input 90: "Sub-Network Input #91" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_91_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(90, (target.get_id(), output_index));
        self
    }

    /// Connects to input 91: "Sub-Network Input #92"
    pub fn set_input_sub_network_input_92(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(91, (target.get_id(), 0));
        self
    }

    /// Connects to input 91: "Sub-Network Input #92" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_92_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(91, (target.get_id(), output_index));
        self
    }

    /// Connects to input 92: "Sub-Network Input #93"
    pub fn set_input_sub_network_input_93(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(92, (target.get_id(), 0));
        self
    }

    /// Connects to input 92: "Sub-Network Input #93" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_93_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(92, (target.get_id(), output_index));
        self
    }

    /// Connects to input 93: "Sub-Network Input #94"
    pub fn set_input_sub_network_input_94(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(93, (target.get_id(), 0));
        self
    }

    /// Connects to input 93: "Sub-Network Input #94" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_94_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(93, (target.get_id(), output_index));
        self
    }

    /// Connects to input 94: "Sub-Network Input #95"
    pub fn set_input_sub_network_input_95(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(94, (target.get_id(), 0));
        self
    }

    /// Connects to input 94: "Sub-Network Input #95" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_95_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(94, (target.get_id(), output_index));
        self
    }

    /// Connects to input 95: "Sub-Network Input #96"
    pub fn set_input_sub_network_input_96(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(95, (target.get_id(), 0));
        self
    }

    /// Connects to input 95: "Sub-Network Input #96" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_96_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(95, (target.get_id(), output_index));
        self
    }

    /// Connects to input 96: "Sub-Network Input #97"
    pub fn set_input_sub_network_input_97(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(96, (target.get_id(), 0));
        self
    }

    /// Connects to input 96: "Sub-Network Input #97" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_97_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(96, (target.get_id(), output_index));
        self
    }

    /// Connects to input 97: "Sub-Network Input #98"
    pub fn set_input_sub_network_input_98(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(97, (target.get_id(), 0));
        self
    }

    /// Connects to input 97: "Sub-Network Input #98" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_98_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(97, (target.get_id(), output_index));
        self
    }

    /// Connects to input 98: "Sub-Network Input #99"
    pub fn set_input_sub_network_input_99(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(98, (target.get_id(), 0));
        self
    }

    /// Connects to input 98: "Sub-Network Input #99" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_99_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(98, (target.get_id(), output_index));
        self
    }

    /// Connects to input 99: "Sub-Network Input #100"
    pub fn set_input_sub_network_input_100(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(99, (target.get_id(), 0));
        self
    }

    /// Connects to input 99: "Sub-Network Input #100" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_100_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(99, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotblend(mut self, val: ChopConstraintsequenceRotblend) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rotblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_writemask(mut self, val: ChopConstraintsequenceWritemask) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintsequenceVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintsequenceVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintsequenceVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintsequenceSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintsequenceUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintsequence {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintsequence"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsimpleblendRotblend {
    /// Euler (Fixed Angles)
    EulerFixedAngles = 0,
    ShortestPath = 1,
    /// Euler (Raw Angles)
    EulerRawAngles = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsimpleblendWritemask {
    Tx = 0,
    Ty = 1,
    Tz = 2,
    Rx = 3,
    Ry = 4,
    Rz = 5,
    Sx = 6,
    Sy = 7,
    Sz = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsimpleblendVexAlign {
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
pub enum ChopConstraintsimpleblendVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsimpleblendVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsimpleblendSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsimpleblendUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintsimpleblend {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintsimpleblend {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1"
    pub fn set_input_sub_network_input_1(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Sub-Network Input #1" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2"
    pub fn set_input_sub_network_input_2(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Sub-Network Input #2" and specifies the output index of the target node.
    pub fn set_input_sub_network_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_blend(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_rotblend(mut self, val: ChopConstraintsimpleblendRotblend) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_rotblend_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "rotblend".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_writemask(mut self, val: ChopConstraintsimpleblendWritemask) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_writemask_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "writemask".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintsimpleblendVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintsimpleblendVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintsimpleblendVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintsimpleblendSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintsimpleblendUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintsimpleblend {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintsimpleblend"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceMode {
    /// UV (Sticky)
    UvSticky = 0,
    /// Primitive + UV
    PrimitivePlusUv = 1,
    PointGroup = 2,
    PrimitiveGroup = 3,
    ClosestDistance = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceLookatmode {
    None = 0,
    DirectionAttributeFromGeometry = 1,
    UDirection = 2,
    VDirection = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceLookupmode {
    UpVector = 0,
    UpVectorAttributeFromGeometry = 1,
    UDirection = 2,
    VDirection = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceLookataxis {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// Z-
    ZMinus = 2,
    /// X+
    XPlus = 3,
    /// Y+
    YPlus = 4,
    /// Z+
    ZPlus = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceLookupaxisx {
    /// Y-
    YMinus = 0,
    /// Z-
    ZMinus = 1,
    /// Y+
    YPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceLookupaxisy {
    /// X-
    XMinus = 0,
    /// Z-
    ZMinus = 1,
    /// X+
    XPlus = 2,
    /// Z+
    ZPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceLookupaxisz {
    /// X-
    XMinus = 0,
    /// Y-
    YMinus = 1,
    /// X+
    XPlus = 2,
    /// Y+
    YPlus = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceVexAlign {
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
pub enum ChopConstraintsurfaceVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstraintsurfaceUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstraintsurface {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstraintsurface {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input"
    pub fn set_input_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input" and specifies the output index of the target node.
    pub fn set_input_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Lookat"
    pub fn set_input_lookat(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Lookat" and specifies the output index of the target node.
    pub fn set_input_lookat_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Up"
    pub fn set_input_up(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Up" and specifies the output index of the target node.
    pub fn set_input_up_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    /// Connects to input 3: "Coordinates"
    pub fn set_input_coordinates(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(3, (target.get_id(), 0));
        self
    }

    /// Connects to input 3: "Coordinates" and specifies the output index of the target node.
    pub fn set_input_coordinates_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(3, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_searchdist(mut self, val: f32) -> Self {
        self.base.params.insert(
            "searchdist".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_searchdist_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "searchdist".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_roll(mut self, val: f32) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roll_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "roll".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_uv(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "uv".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_uv_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uv".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upvector(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_upvector_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upvector".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_searchmaxpnt(mut self, val: i32) -> Self {
        self.base.params.insert(
            "searchmaxpnt".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_searchmaxpnt_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "searchmaxpnt".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_mode(mut self, val: ChopConstraintsurfaceMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookatmode(mut self, val: ChopConstraintsurfaceLookatmode) -> Self {
        self.base.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookatmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookatmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupmode(mut self, val: ChopConstraintsurfaceLookupmode) -> Self {
        self.base.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookataxis(mut self, val: ChopConstraintsurfaceLookataxis) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookataxis_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookataxis".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisx(mut self, val: ChopConstraintsurfaceLookupaxisx) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisx_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisx".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisy(mut self, val: ChopConstraintsurfaceLookupaxisy) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisy_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisy".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_lookupaxisz(mut self, val: ChopConstraintsurfaceLookupaxisz) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_lookupaxisz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "lookupaxisz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_align(mut self, val: ChopConstraintsurfaceVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstraintsurfaceVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstraintsurfaceVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstraintsurfaceSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstraintsurfaceUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "soppath".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_group(mut self, val: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_group_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "group".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_uvattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "uvattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_uvattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "uvattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "pattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_pattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_dirattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_dirattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "dirattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_upattribute(mut self, val: &str) -> Self {
        self.base.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_upattribute_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "upattribute".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_subdi(mut self, val: bool) -> Self {
        self.base.params.insert(
            "subdi".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_subdi_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "subdi".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstraintsurface {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constraintsurface"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformTrs {
    ScaleRotTrans = 0,
    ScaleTransRot = 1,
    RotScaleTrans = 2,
    RotTransScale = 3,
    TransScaleRot = 4,
    TransRotScale = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformXyz {
    RxRyRz = 0,
    RxRzRy = 1,
    RyRxRz = 2,
    RyRzRx = 3,
    RzRxRy = 4,
    RzRyRx = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformMode {
    /// Post-Multiply First Input
    PostMinusMultiplyFirstInput = 0,
    /// Pre-Multiply First Input
    PreMinusMultiplyFirstInput = 1,
    ReturnFirstInput = 2,
    ReturnSecondInput = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformPmode {
    /// Pivot from Input#3. Pivot Rotate from Parameters
    PivotFromInput3PivotRotateFromParameters = 0,
    /// Pivot from Parameters. Pivot Rotate from Input #3
    PivotFromParametersPivotRotateFromInput3 = 1,
    /// Both from Input #3
    BothFromInput3 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformVexAlign {
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
pub enum ChopConstrainttransformVexRange {
    UseFullAnimationRange = 0,
    UseCurrentFrame = 1,
    /// Use Start/End
    UseStartEnd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformVexNumThreads {
    NoThreading = 0,
    /// 1 Per Processor
    N1PerProcessor = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopConstrainttransformUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopConstrainttransform {
    pub base: crate::core::types::NodeBase,
}

impl ChopConstrainttransform {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Input 2"
    pub fn set_input_input_2(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Input 2" and specifies the output index of the target node.
    pub fn set_input_input_2_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Pivot"
    pub fn set_input_pivot(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Pivot" and specifies the output index of the target node.
    pub fn set_input_pivot_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_vex_edit(mut self) -> Self {
        self.base.params.insert(
            "vex_edit".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_vex_reload(mut self) -> Self {
        self.base.params.insert(
            "vex_reload".to_string(),
            crate::core::types::ParamValue::Button,
        );
        self
    }

    // --- Float parameters ---
    pub fn with_vex_start(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_start_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_start".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_end(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_end_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_end".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_rate(mut self, val: f32) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_vex_rate_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_rate".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_t(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("t".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_t_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "t".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_r(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("r".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_r_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "r".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_s(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("s".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_s_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "s".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_p(mut self, val: [f32; 3]) -> Self {
        self.base
            .params
            .insert("p".to_string(), crate::core::types::ParamValue::Float3(val));
        self
    }
    pub fn with_p_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "p".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pr(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_pr_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pr".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_iterate_over_channels(mut self, val: i32) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_iterate_over_channels_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "__iterate_over_channels__".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_trs(mut self, val: ChopConstrainttransformTrs) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_trs_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "trs".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_xyz(mut self, val: ChopConstrainttransformXyz) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_xyz_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "xyz".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_mode(mut self, val: ChopConstrainttransformMode) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_pmode(mut self, val: ChopConstrainttransformPmode) -> Self {
        self.base.params.insert(
            "pmode".to_string(),
            crate::core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_pmode_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pmode".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_vex_align(mut self, val: ChopConstrainttransformVexAlign) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_align_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_align".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_range(mut self, val: ChopConstrainttransformVexRange) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_range_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_range".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_num_threads(mut self, val: ChopConstrainttransformVexNumThreads) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_vex_num_threads_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_num_threads".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopConstrainttransformSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopConstrainttransformUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_vex_name(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_name_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_name".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vex_precision(mut self, val: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_vex_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vex_precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_invert(mut self, val: bool) -> Self {
        self.base.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "invert".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopConstrainttransform {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "constrainttransform"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone)]
pub struct ChopCop2net {
    pub base: crate::core::types::NodeBase,
}

impl ChopCop2net {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }
}

impl crate::core::types::HoudiniNode for ChopCop2net {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "cop2net"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCopnetBorder {
    Constant = 0,
    Clamp = 1,
    Mirror = 2,
    Wrap = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCopnetPrecision {
    /// 16-bit
    N16MinusBit = 0,
    /// 32-bit
    N32MinusBit = 1,
}

#[derive(Debug, Clone)]
pub struct ChopCopnet {
    pub base: crate::core::types::NodeBase,
}

impl ChopCopnet {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Float parameters ---
    pub fn with_pixelscale(mut self, val: f32) -> Self {
        self.base.params.insert(
            "pixelscale".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_pixelscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "pixelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int parameters ---
    pub fn with_udim(mut self, val: i32) -> Self {
        self.base
            .params
            .insert("udim".to_string(), crate::core::types::ParamValue::Int(val));
        self
    }
    pub fn with_udim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "udim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_vistile(mut self, val: i32) -> Self {
        self.base.params.insert(
            "vistile".to_string(),
            crate::core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_vistile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "vistile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Int2 parameters ---
    pub fn with_res(mut self, val: [i32; 2]) -> Self {
        self.base
            .params
            .insert("res".to_string(), crate::core::types::ParamValue::Int2(val));
        self
    }
    pub fn with_res_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "res".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_resmenu(mut self, val: i32) -> Self {
        self.base.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Menu(val),
        );
        self
    }
    pub fn with_resmenu_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "resmenu".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_border(mut self, val: ChopCopnetBorder) -> Self {
        self.base.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "border".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_precision(mut self, val: ChopCopnetPrecision) -> Self {
        self.base.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_precision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "precision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_setres(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setres".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setres_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setres".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setpixelscale(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setpixelscale".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setpixelscale_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setpixelscale".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setborder(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setborder".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setborder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setborder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setprecision(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setprecision_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setprecision".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setudim(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setudim_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setudim".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_setvistile(mut self, val: bool) -> Self {
        self.base.params.insert(
            "setvistile".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setvistile_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "setvistile".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopCopnet {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "copnet"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCopyMethod {
    TriggeredCopy = 0,
    Convolve = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCopyOutput {
    OneChannelPerTemplateChannel = 0,
    OneChannelPerCopyChannel = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCopyRemainder {
    DiscardRemainder = 0,
    MakeOutputLonger = 1,
    MixRemainderWithBeginning = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCopySrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCopyUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopCopy {
    pub base: crate::core::types::NodeBase,
}

impl ChopCopy {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Template"
    pub fn set_input_template(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Template" and specifies the output index of the target node.
    pub fn set_input_template_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Copy"
    pub fn set_input_copy(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Copy" and specifies the output index of the target node.
    pub fn set_input_copy_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_threshold(mut self, val: f32) -> Self {
        self.base.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_threshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val1(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val1".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val2(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val2".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val3(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val3".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val4(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val4".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val5(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val5".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val5_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val6(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val6".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val6_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val7(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val7".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val7_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val8(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val8".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val8_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val9(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val9".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val9_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_val10(mut self, val: f32) -> Self {
        self.base.params.insert(
            "val10".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_val10_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "val10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_method(mut self, val: ChopCopyMethod) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "method".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output(mut self, val: ChopCopyOutput) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_remainder(mut self, val: ChopCopyRemainder) -> Self {
        self.base.params.insert(
            "remainder".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_remainder_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "remainder".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopCopySrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopCopyUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_param1(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param1".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param1_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param1".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param2(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param2".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param2_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param2".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param3(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param3".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param3_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param3".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param4(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param4".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param4_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param4".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param5(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param5".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param5_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param5".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param6(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param6".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param6_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param6".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param7(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param7".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param7_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param7".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param8(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param8".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param8_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param8".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param9(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param9".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param9_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param9".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_param10(mut self, val: &str) -> Self {
        self.base.params.insert(
            "param10".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_param10_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "param10".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_keep(mut self, val: bool) -> Self {
        self.base.params.insert(
            "keep".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_keep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "keep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_cook(mut self, val: bool) -> Self {
        self.base.params.insert(
            "cook".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_cook_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "cook".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopCopy {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "copy"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountTriggeron {
    IncreasingValues = 0,
    DecreasingValues = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountOutput {
    Off = 0,
    /// Loop Min/Max
    LoopMinMax = 1,
    /// Clamp Min/Max
    ClampMinMax = 2,
    /// Loop Min, Clamp Max
    LoopMinClampMax = 3,
    /// Clamp Min, Loop Max
    ClampMinLoopMax = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountOffon {
    None = 0,
    IncreaseCount = 1,
    DecreaseCount = 2,
    IncreaseCountByTime = 3,
    DecreaseCountByTime = 4,
    ResetCountToZero = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountOn {
    None = 0,
    IncreaseCount = 1,
    DecreaseCount = 2,
    IncreaseCountByTime = 3,
    DecreaseCountByTime = 4,
    ResetCountToZero = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountOnoff {
    None = 0,
    IncreaseCount = 1,
    DecreaseCount = 2,
    IncreaseCountByTime = 3,
    DecreaseCountByTime = 4,
    ResetCountToZero = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountOff {
    None = 0,
    IncreaseCount = 1,
    DecreaseCount = 2,
    IncreaseCountByTime = 3,
    DecreaseCountByTime = 4,
    ResetCountToZero = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCountUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopCount {
    pub base: crate::core::types::NodeBase,
}

impl ChopCount {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Channels to Count"
    pub fn set_input_channels_to_count(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Channels to Count" and specifies the output index of the target node.
    pub fn set_input_channels_to_count_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 1: "Reset Pulses"
    pub fn set_input_reset_pulses(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(1, (target.get_id(), 0));
        self
    }

    /// Connects to input 1: "Reset Pulses" and specifies the output index of the target node.
    pub fn set_input_reset_pulses_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(1, (target.get_id(), output_index));
        self
    }

    /// Connects to input 2: "Increment Value"
    pub fn set_input_increment_value(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), 0));
        self
    }

    /// Connects to input 2: "Increment Value" and specifies the output index of the target node.
    pub fn set_input_increment_value_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(2, (target.get_id(), output_index));
        self
    }

    // --- Button parameters ---
    pub fn trigger_reset(mut self) -> Self {
        self.base
            .params
            .insert("reset".to_string(), crate::core::types::ParamValue::Button);
        self
    }

    // --- Float parameters ---
    pub fn with_threshup(mut self, val: f32) -> Self {
        self.base.params.insert(
            "threshup".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_threshup_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "threshup".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_threshdown(mut self, val: f32) -> Self {
        self.base.params.insert(
            "threshdown".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_threshdown_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "threshdown".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_retrigger(mut self, val: f32) -> Self {
        self.base.params.insert(
            "retrigger".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_retrigger_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "retrigger".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitmin(mut self, val: f32) -> Self {
        self.base.params.insert(
            "limitmin".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_limitmin_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitmin".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_limitmax(mut self, val: f32) -> Self {
        self.base.params.insert(
            "limitmax".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_limitmax_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "limitmax".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_triggeron(mut self, val: ChopCountTriggeron) -> Self {
        self.base.params.insert(
            "triggeron".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_triggeron_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "triggeron".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_output(mut self, val: ChopCountOutput) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_output_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "output".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_offon(mut self, val: ChopCountOffon) -> Self {
        self.base.params.insert(
            "offon".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_offon_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "offon".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_on(mut self, val: ChopCountOn) -> Self {
        self.base.params.insert(
            "on".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_on_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "on".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_onoff(mut self, val: ChopCountOnoff) -> Self {
        self.base.params.insert(
            "onoff".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_onoff_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "onoff".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_off(mut self, val: ChopCountOff) -> Self {
        self.base.params.insert(
            "off".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_off_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "off".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopCountSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopCountUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_threshold(mut self, val: bool) -> Self {
        self.base.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_threshold_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "threshold".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopCount {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "count"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCycleBlendmethod {
    PreserveLength = 0,
    OverlapSequences = 1,
    InsertBlendRegion = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCycleBlendfunc {
    Linear = 0,
    EaseIn = 1,
    EaseOut = 2,
    EaseInEaseOut = 3,
    Cubic = 4,
    Add = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCycleSrselect {
    /// Resample At First Input's Rate
    ResampleAtFirstInputSRate = 0,
    ResampleAtMaximumRate = 1,
    ResampleAtMinimumRate = 2,
    ErrorIfRatesDiffer = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChopCycleUnits {
    Frames = 0,
    Samples = 1,
    Seconds = 2,
}

#[derive(Debug, Clone)]
pub struct ChopCycle {
    pub base: crate::core::types::NodeBase,
}

impl ChopCycle {
    pub fn new(name: &str) -> Self {
        Self {
            base: crate::core::types::NodeBase::new(name),
        }
    }

    // --- Spare Parameters ---
    pub fn add_spare<S: Into<crate::core::types::SpareParam>>(mut self, spare: S) -> Self {
        self.base.spare_params.push(spare.into());
        self
    }

    // --- Inputs ---
    /// Manually connects to a specific input index.
    pub fn set_input_at(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
    ) -> Self {
        self.base.inputs.insert(index, (target.get_id(), 0));
        self
    }

    /// Manually connects to a specific input index and specifies the output index of the target node.
    pub fn set_input_at_from(
        mut self,
        index: usize,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base
            .inputs
            .insert(index, (target.get_id(), output_index));
        self
    }

    /// Connects to the primary input (index 0).
    pub fn set_input(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to the primary input (index 0) and specifies the output index of the target node.
    pub fn set_input_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    /// Connects to input 0: "Input 1"
    pub fn set_input_input_1(mut self, target: &dyn crate::core::types::HoudiniNode) -> Self {
        self.base.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Input 1" and specifies the output index of the target node.
    pub fn set_input_input_1_from(
        mut self,
        target: &dyn crate::core::types::HoudiniNode,
        output_index: usize,
    ) -> Self {
        self.base.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_before(mut self, val: f32) -> Self {
        self.base.params.insert(
            "before".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_before_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "before".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_after(mut self, val: f32) -> Self {
        self.base.params.insert(
            "after".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_after_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "after".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendregion(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blendregion".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendregion_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendregion".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendbias(mut self, val: f32) -> Self {
        self.base.params.insert(
            "blendbias".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendbias_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendbias".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_step(mut self, val: f32) -> Self {
        self.base.params.insert(
            "step".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_step_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "step".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_gcolorstep(mut self, val: f32) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gcolorstep_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolorstep".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_gcolor(mut self, val: [f32; 3]) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_gcolor_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "gcolor".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_blendmethod(mut self, val: ChopCycleBlendmethod) -> Self {
        self.base.params.insert(
            "blendmethod".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendmethod_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendmethod".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_blendfunc(mut self, val: ChopCycleBlendfunc) -> Self {
        self.base.params.insert(
            "blendfunc".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_blendfunc_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "blendfunc".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_srselect(mut self, val: ChopCycleSrselect) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_srselect_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "srselect".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_units(mut self, val: ChopCycleUnits) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_units_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "units".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- String parameters ---
    pub fn with_stepscope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "stepscope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_stepscope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "stepscope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_scope(mut self, val: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_scope_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "scope".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_export(mut self, val: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::String(val.to_string()),
        );
        self
    }
    pub fn with_export_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "export".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "mirror".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_extremes(mut self, val: bool) -> Self {
        self.base.params.insert(
            "extremes".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_extremes_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "extremes".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_timeslice(mut self, val: bool) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_timeslice_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "timeslice".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
    pub fn with_unload(mut self, val: bool) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_unload_expr(mut self, expr: &str) -> Self {
        self.base.params.insert(
            "unload".to_string(),
            crate::core::types::ParamValue::Expression(expr.to_string()),
        );
        self
    }
}

impl crate::core::types::HoudiniNode for ChopCycle {
    fn get_id(&self) -> usize {
        self.base.id
    }

    fn get_name(&self) -> &str {
        &self.base.name
    }

    fn get_node_type(&self) -> &'static str {
        "cycle"
    }

    fn get_inputs(&self) -> &std::collections::BTreeMap<usize, (usize, usize)> {
        &self.base.inputs
    }

    fn get_params(&self) -> &std::collections::HashMap<String, crate::core::types::ParamValue> {
        &self.base.params
    }

    fn get_spare_params(&self) -> &[crate::core::types::SpareParam] {
        &self.base.spare_params
    }
}
