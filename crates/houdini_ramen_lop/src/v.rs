#[derive(Debug, Clone)]
pub struct LopValueclip {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopValueclip {
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

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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

    pub fn with_starttime(mut self, val: f32) -> Self {
        self.params.insert(
            "starttime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_starttime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "starttime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipstarttime(mut self, val: f32) -> Self {
        self.params.insert(
            "clipstarttime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_clipstarttime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipstarttime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cliptimescale(mut self, val: f32) -> Self {
        self.params.insert(
            "cliptimescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cliptimescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cliptimescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loopendtime(mut self, val: f32) -> Self {
        self.params.insert(
            "loopendtime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_loopendtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "loopendtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_duration_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("duration{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_duration_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("duration{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loopcount_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("loopcount{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_loopcount_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("loopcount{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loopstart_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("loopstart{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_loopstart_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("loopstart{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_loopstep_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("loopstep{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_loopstep_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("loopstep{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipset(mut self, val: &str) -> Self {
        self.params.insert(
            "clipset".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipprimpath(mut self, val: &str) -> Self {
        self.params.insert(
            "clipprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipprimpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "clipprimpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_manifestfile(mut self, val: &str) -> Self {
        self.params.insert(
            "manifestfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_manifestfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "manifestfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipfile_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("clipfile{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_clipfile_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("clipfile{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setclipstarttime(mut self, val: bool) -> Self {
        self.params.insert(
            "setclipstarttime".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setclipstarttime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setclipstarttime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setcliptimescale(mut self, val: bool) -> Self {
        self.params.insert(
            "setcliptimescale".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setcliptimescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setcliptimescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setloopendtime(mut self, val: bool) -> Self {
        self.params.insert(
            "setloopendtime".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setloopendtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setloopendtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_firstandlastframesmatch(mut self, val: bool) -> Self {
        self.params.insert(
            "firstandlastframesmatch".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_firstandlastframesmatch_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "firstandlastframesmatch".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_setloopcount_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("setloopcount{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setloopcount_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("setloopcount{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopValueclip {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "valueclip"
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

pub trait LopValueclipOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopValueclipOutputs for LopValueclip {}
impl LopValueclipOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopValueclip> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopVarymaterialassignmentBindstrength {
    Default = 0,
    StrongerThanDescendants = 1,
    WeakerThanDescendants = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopVarymaterialassignmentBindmethod {
    Direct = 0,
    CollectionBased = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LopVarymaterialassignmentMethod {
    Random = 0,
    SpatialDistribution = 1,
    WeightedRandom = 2,
}

#[derive(Debug, Clone)]
pub struct LopVarymaterialassignment {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopVarymaterialassignment {
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

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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

    pub fn trigger_weighted_populate(mut self) -> Self {
        self.params.insert(
            "weighted_populate".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_collection_populate(mut self) -> Self {
        self.params.insert(
            "collection_populate".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_random_seed(mut self, val: f32) -> Self {
        self.params.insert(
            "random_seed".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_random_seed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "random_seed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_frame(mut self, val: f32) -> Self {
        self.params.insert(
            "spatial_frame".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spatial_frame_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_frame".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_time(mut self, val: f32) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_time_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "time".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_noiseamp(mut self, val: f32) -> Self {
        self.params.insert(
            "spatial_noiseamp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spatial_noiseamp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_noiseamp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_noiserough(mut self, val: f32) -> Self {
        self.params.insert(
            "spatial_noiserough".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_spatial_noiserough_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_noiserough".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weighted_matweight_inst(mut self, index1: usize, val: f32) -> Self {
        self.params.insert(
            format!("weighted_matweight{}", index1),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_weighted_matweight_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("weighted_matweight{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_cellfreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spatial_cellfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spatial_cellfreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_cellfreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_celloffset(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spatial_celloffset".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spatial_celloffset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_celloffset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_celljitter(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spatial_celljitter".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spatial_celljitter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_celljitter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_noisefreq(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "spatial_noisefreq".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_spatial_noisefreq_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_noisefreq".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_method(mut self, val: LopVarymaterialassignmentMethod) -> Self {
        self.params.insert(
            "method".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "method".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_noisemaxoctave(mut self, val: i32) -> Self {
        self.params.insert(
            "spatial_noisemaxoctave".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_spatial_noisemaxoctave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_noisemaxoctave".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindstrength(mut self, val: LopVarymaterialassignmentBindstrength) -> Self {
        self.params.insert(
            "bindstrength".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindstrength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindstrength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindmethod(mut self, val: LopVarymaterialassignmentBindmethod) -> Self {
        self.params.insert(
            "bindmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_bindmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bias(mut self, val: Vec<houdini_ramen_core::types::RampPoint>) -> Self {
        self.params.insert(
            "bias".to_string(),
            houdini_ramen_core::types::ParamValue::Ramp(val),
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
    pub fn with_primitives(mut self, val: &str) -> Self {
        self.params.insert(
            "primitives".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primitives_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primitives".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_materials(mut self, val: &str) -> Self {
        self.params.insert(
            "materials".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_materials_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materials".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindpurpose(mut self, val: &str) -> Self {
        self.params.insert(
            "bindpurpose".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindpurpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindpurpose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindpath(mut self, val: &str) -> Self {
        self.params.insert(
            "bindpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionpath(mut self, val: &str) -> Self {
        self.params.insert(
            "collectionpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collectionpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionname(mut self, val: &str) -> Self {
        self.params.insert(
            "collectionname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collectionname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collectionname".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_method(mut self, val: &str) -> Self {
        self.params.insert(
            "spatial_method".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_spatial_method_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "spatial_method".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weighted_material_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("weighted_material{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weighted_material_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("weighted_material{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_root(mut self, val: &str) -> Self {
        self.params.insert(
            "collection_root".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_root_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collection_root".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_geocoll_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_geocoll{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_geocoll_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_geocoll{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_matcoll_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_matcoll{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_matcoll_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_matcoll{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_matpath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("collection_matpath{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_matpath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_matpath{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bindcollectionexpand(mut self, val: bool) -> Self {
        self.params.insert(
            "bindcollectionexpand".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bindcollectionexpand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bindcollectionexpand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_integerframes(mut self, val: bool) -> Self {
        self.params.insert(
            "integerframes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_integerframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "integerframes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_weighted_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("weighted_enable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_weighted_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("weighted_enable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_enable_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("collection_enable{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collection_enable_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_enable{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collection_usepath_inst(mut self, index1: usize, val: bool) -> Self {
        self.params.insert(
            format!("collection_usepath{}", index1),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_collection_usepath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("collection_usepath{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopVarymaterialassignment {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "varymaterialassignment"
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

pub trait LopVarymaterialassignmentOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopVarymaterialassignmentOutputs for LopVarymaterialassignment {}
impl LopVarymaterialassignmentOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<LopVarymaterialassignment>
{
}

#[derive(Debug, Clone)]
pub struct LopVolume {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopVolume {
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

    pub fn set_input_stage_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
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

    pub fn with_sample_shutterrange(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_sample_shutterrange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shutterrange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_sample_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_count(mut self, val: i32) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sample_count_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_count".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_behavior_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_behavior".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_shuttermode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_shuttermode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim(mut self, val: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_cameraprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_cameraprim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath(mut self, val: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind(mut self, val: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primkind_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primkind".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype(mut self, val: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parentprimtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parentprimtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier(mut self, val: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_specifier_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "specifier".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filepath_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("filepath{}", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_filepath_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("filepath{}", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_fields_inst(mut self, index1: usize, val: &str) -> Self {
        self.params.insert(
            format!("file{}_fields", index1),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_fields_inst_expr(mut self, index1: usize, expr: &str) -> Self {
        self.params.insert(
            format!("file{}_fields", index1),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_srcname_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("file{}_srcname{}", index1, index2),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_srcname_inst_expr(mut self, index1: usize, index2: usize, expr: &str) -> Self {
        self.params.insert(
            format!("file{}_srcname{}", index1, index2),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_destname_inst(mut self, index1: usize, index2: usize, val: &str) -> Self {
        self.params.insert(
            format!("file{}_destname{}", index1, index2),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_file_destname_inst_expr(
        mut self,
        index1: usize,
        index2: usize,
        expr: &str,
    ) -> Self {
        self.params.insert(
            format!("file{}_destname{}", index1, index2),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_subframeenable(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_subframeenable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_subframeenable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sample_includeframe(mut self, val: bool) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_sample_includeframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sample_includeframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_errorformissingfile(mut self, val: bool) -> Self {
        self.params.insert(
            "errorformissingfile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_errorformissingfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "errorformissingfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for LopVolume {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "volume"
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

pub trait LopVolumeOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output Stage"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl LopVolumeOutputs for LopVolume {}
impl LopVolumeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<LopVolume> {}

#[derive(Debug, Clone)]
pub struct LopVopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl LopVopnet {
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

impl houdini_ramen_core::types::HoudiniNode for LopVopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "vopnet"
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
