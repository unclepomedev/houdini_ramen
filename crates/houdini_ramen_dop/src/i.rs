#[derive(Debug, Clone)]
pub struct DopImpactanalysis {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopImpactanalysis {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_time_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "time_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_impact_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "impact_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impact_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impact_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distance_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "distance_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distance_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distance_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- String parameters ---
    pub fn with_force_object(mut self, val: &str) -> Self {
        self.params.insert(
            "force_object".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_force_object_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "force_object".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_display(mut self, val: bool) -> Self {
        self.params.insert(
            "display".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
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
    pub fn with_addaffectors(mut self, val: bool) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addaffectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addaffectors".to_string(),
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
    pub fn with_solverperobject(mut self, val: bool) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solverperobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solverperobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopImpactanalysis {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "impactanalysis"
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
pub enum DopImpulseforceParmopImpulse {
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
pub enum DopImpulseforceParmopHandlepos {
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
pub enum DopImpulseforceParmopSamplemode {
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
pub enum DopImpulseforceSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopImpulseforceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopImpulseforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopImpulseforce {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DopImpulseforce {
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

    // --- Float3 parameters ---
    pub fn with_impulse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "impulse".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_impulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_handlepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "handlepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_handlepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "handlepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
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

    // --- Menu parameters ---
    pub fn with_parmop_impulse(mut self, val: DopImpulseforceParmopImpulse) -> Self {
        self.params.insert(
            "parmop_impulse".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_impulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_impulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_handlepos(mut self, val: DopImpulseforceParmopHandlepos) -> Self {
        self.params.insert(
            "parmop_handlepos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_handlepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_handlepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_samplemode(mut self, val: DopImpulseforceParmopSamplemode) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_samplemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_samplemode(mut self, val: DopImpulseforceSamplemode) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_samplemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "samplemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopImpulseforceDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopImpulseforceSharedata) -> Self {
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

    // --- String parameters ---
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

    // --- Toggle parameters ---
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

impl houdini_ramen_core::types::HoudiniNode for DopImpulseforce {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "impulseforce"
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
pub enum DopIndexfieldParmopTwod {
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
pub enum DopIndexfieldParmopVoxelplane {
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
pub enum DopIndexfieldVoxelplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIndexfieldParmopUniformvoxels {
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
pub enum DopIndexfieldUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIndexfieldParmopUniformdiv {
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
pub enum DopIndexfieldParmopDiv {
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
pub enum DopIndexfieldParmopDivsize {
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
pub enum DopIndexfieldParmopSize {
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
pub enum DopIndexfieldParmopT {
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
pub enum DopIndexfieldParmopVoxelsample {
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
pub enum DopIndexfieldVoxelsample {
    Center = 0,
    FaceX = 1,
    FaceY = 2,
    FaceZ = 3,
    EdgeXy = 4,
    EdgeXz = 5,
    EdgeYz = 6,
    Corner = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIndexfieldParmopBorder {
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
pub enum DopIndexfieldBorder {
    Constant = 0,
    Repeat = 1,
    Streak = 2,
    Mirror = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIndexfieldParmopInitialvalue {
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
pub enum DopIndexfieldParmopSlice {
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
pub enum DopIndexfieldParmopSlicediv {
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
pub enum DopIndexfieldParmopSliceoverlapneg {
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
pub enum DopIndexfieldParmopSliceoverlappos {
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
pub enum DopIndexfieldParmopPositionpath {
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
pub enum DopIndexfieldDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIndexfieldSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopIndexfield {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DopIndexfield {
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
    pub fn with_divsize(mut self, val: f32) -> Self {
        self.params.insert(
            "divsize".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "divsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_size(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "size".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "size".to_string(),
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

    // --- Int parameters ---
    pub fn with_uniformdiv(mut self, val: i32) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initialvalue(mut self, val: i32) -> Self {
        self.params.insert(
            "initialvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_initialvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_slice(mut self, val: i32) -> Self {
        self.params.insert(
            "slice".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slice".to_string(),
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

    // --- Int3 parameters ---
    pub fn with_div(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "div".to_string(),
            houdini_ramen_core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "div".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_slicediv(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            houdini_ramen_core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_slicediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slicediv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sliceoverlapneg(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            houdini_ramen_core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlapneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlapneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sliceoverlappos(mut self, val: [i32; 3]) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            houdini_ramen_core::types::ParamValue::Int3(val),
        );
        self
    }
    pub fn with_sliceoverlappos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sliceoverlappos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Menu parameters ---
    pub fn with_parmop_twod(mut self, val: DopIndexfieldParmopTwod) -> Self {
        self.params.insert(
            "parmop_twod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_twod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_voxelplane(mut self, val: DopIndexfieldParmopVoxelplane) -> Self {
        self.params.insert(
            "parmop_voxelplane".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_voxelplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_voxelplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_voxelplane(mut self, val: DopIndexfieldVoxelplane) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voxelplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_uniformvoxels(mut self, val: DopIndexfieldParmopUniformvoxels) -> Self {
        self.params.insert(
            "parmop_uniformvoxels".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_uniformvoxels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniformvoxels(mut self, val: DopIndexfieldUniformvoxels) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_uniformvoxels_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniformvoxels".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_uniformdiv(mut self, val: DopIndexfieldParmopUniformdiv) -> Self {
        self.params.insert(
            "parmop_uniformdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_uniformdiv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_uniformdiv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_div(mut self, val: DopIndexfieldParmopDiv) -> Self {
        self.params.insert(
            "parmop_div".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_div_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_div".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_divsize(mut self, val: DopIndexfieldParmopDivsize) -> Self {
        self.params.insert(
            "parmop_divsize".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_divsize_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_divsize".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_size(mut self, val: DopIndexfieldParmopSize) -> Self {
        self.params.insert(
            "parmop_size".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_t(mut self, val: DopIndexfieldParmopT) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_t_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_t".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_voxelsample(mut self, val: DopIndexfieldParmopVoxelsample) -> Self {
        self.params.insert(
            "parmop_voxelsample".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_voxelsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_voxelsample".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_voxelsample(mut self, val: DopIndexfieldVoxelsample) -> Self {
        self.params.insert(
            "voxelsample".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_voxelsample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "voxelsample".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_border(mut self, val: DopIndexfieldParmopBorder) -> Self {
        self.params.insert(
            "parmop_border".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_border".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_border(mut self, val: DopIndexfieldBorder) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_border_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "border".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_initialvalue(mut self, val: DopIndexfieldParmopInitialvalue) -> Self {
        self.params.insert(
            "parmop_initialvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_initialvalue_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_initialvalue".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_slice(mut self, val: DopIndexfieldParmopSlice) -> Self {
        self.params.insert(
            "parmop_slice".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_slice_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_slice".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_slicediv(mut self, val: DopIndexfieldParmopSlicediv) -> Self {
        self.params.insert(
            "parmop_slicediv".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_slicediv_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_slicediv".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_sliceoverlapneg(mut self, val: DopIndexfieldParmopSliceoverlapneg) -> Self {
        self.params.insert(
            "parmop_sliceoverlapneg".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_sliceoverlapneg_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_sliceoverlapneg".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_sliceoverlappos(mut self, val: DopIndexfieldParmopSliceoverlappos) -> Self {
        self.params.insert(
            "parmop_sliceoverlappos".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_sliceoverlappos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_sliceoverlappos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_positionpath(mut self, val: DopIndexfieldParmopPositionpath) -> Self {
        self.params.insert(
            "parmop_positionpath".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_positionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_positionpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopIndexfieldDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopIndexfieldSharedata) -> Self {
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

    // --- String parameters ---
    pub fn with_positionpath(mut self, val: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionpath".to_string(),
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

    // --- Toggle parameters ---
    pub fn with_twod(mut self, val: bool) -> Self {
        self.params.insert(
            "twod".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_twod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "twod".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopIndexfield {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "indexfield"
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
pub enum DopIndexfieldvisualizationGuideplane {
    XyPlane = 0,
    YzPlane = 1,
    ZxPlane = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIndexfieldvisualizationGuidevismode {
    /// Infra-Red
    InfraMinusRed = 0,
    WhiteToRed = 1,
    Grayscale = 2,
    Blackbody = 3,
    /// Two-Tone
    TwoMinusTone = 4,
    UniqueColor = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIndexfieldvisualizationSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopIndexfieldvisualization {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopIndexfieldvisualization {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_guideplaneval(mut self, val: f32) -> Self {
        self.params.insert(
            "guideplaneval".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guideplaneval_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideplaneval".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float2 parameters ---
    pub fn with_guiderange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "guiderange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_guiderange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Float3 parameters ---
    pub fn with_guideplanepos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guideplanepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guideplanepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideplanepos".to_string(),
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

    // --- Int parameters ---
    pub fn with_guideindex(mut self, val: i32) -> Self {
        self.params.insert(
            "guideindex".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guideindex_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideindex".to_string(),
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

    // --- Menu parameters ---
    pub fn with_guideplane(mut self, val: DopIndexfieldvisualizationGuideplane) -> Self {
        self.params.insert(
            "guideplane".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guideplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidevismode(mut self, val: DopIndexfieldvisualizationGuidevismode) -> Self {
        self.params.insert(
            "guidevismode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidevismode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidevismode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sharedata(mut self, val: DopIndexfieldvisualizationSharedata) -> Self {
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

    // --- String parameters ---
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

    // --- Toggle parameters ---
    pub fn with_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usebox(mut self, val: bool) -> Self {
        self.params.insert(
            "usebox".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usebox_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebox".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useboxhash(mut self, val: bool) -> Self {
        self.params.insert(
            "useboxhash".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useboxhash_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useboxhash".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideuseplane(mut self, val: bool) -> Self {
        self.params.insert(
            "guideuseplane".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideuseplane_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideuseplane".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideinvert(mut self, val: bool) -> Self {
        self.params.insert(
            "guideinvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guideinvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideinvert".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopIndexfieldvisualization {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "indexfieldvisualization"
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
pub enum DopInitialoverlaprelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopInitialoverlaprel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DopInitialoverlaprel {
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

    // --- Int parameters ---
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

    // --- Menu parameters ---
    pub fn with_sharedata(mut self, val: DopInitialoverlaprelSharedata) -> Self {
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

    // --- String parameters ---
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
    pub fn with_affected(mut self, val: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affected".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affectors(mut self, val: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_affectors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "affectors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relname(mut self, val: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_uniquerelname(mut self, val: bool) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uniquerelname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uniquerelname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_makemutual(mut self, val: bool) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_makemutual_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "makemutual".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopInitialoverlaprel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "initialoverlaprel"
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
pub struct DopInstancedobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopInstancedobject {
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

    // --- String parameters ---
    pub fn with_sourcesoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "sourcesoppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sourcesoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sourcesoppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Toggle parameters ---
    pub fn with_useobjectxform(mut self, val: bool) -> Self {
        self.params.insert(
            "useobjectxform".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useobjectxform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useobjectxform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_solvefirstframe(mut self, val: bool) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_solvefirstframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solvefirstframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopInstancedobject {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "instancedobject"
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
pub enum DopIntangiblevalueParmopIntangible {
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
pub enum DopIntangiblevalueDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopIntangiblevalueSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopIntangiblevalue {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, usize)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopIntangiblevalue {
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

    /// Connects to input 0: "Objects to be processed"
    pub fn set_input_objects_to_be_processed<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), 0));
        self
    }

    /// Connects to input 0: "Objects to be processed" and specifies the output index of the target node.
    pub fn set_input_objects_to_be_processed_from<N: houdini_ramen_core::types::HoudiniNode>(
        mut self,
        target: &N,
        output_index: usize,
    ) -> Self {
        self.inputs.insert(0, (target.get_id(), output_index));
        self
    }

    // --- Float parameters ---
    pub fn with_intangible(mut self, val: f32) -> Self {
        self.params.insert(
            "intangible".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_intangible_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "intangible".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }

    // --- Int parameters ---
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

    // --- Menu parameters ---
    pub fn with_parmop_intangible(mut self, val: DopIntangiblevalueParmopIntangible) -> Self {
        self.params.insert(
            "parmop_intangible".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_intangible_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_intangible".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopIntangiblevalueDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopIntangiblevalueSharedata) -> Self {
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

    // --- String parameters ---
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

    // --- Toggle parameters ---
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

impl houdini_ramen_core::types::HoudiniNode for DopIntangiblevalue {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_node_type(&self) -> &'static str {
        "intangiblevalue"
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
