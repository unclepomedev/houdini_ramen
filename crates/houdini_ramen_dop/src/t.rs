#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTargetrelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopTargetrel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
    next_input_index: usize,
}

impl DopTargetrel {
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
        self.inputs.insert(index, (out.node_id, out.pin));
        self
    }

    pub fn add_input<O: Into<houdini_ramen_core::types::NodeOutput>>(mut self, output: O) -> Self {
        let out = output.into();
        self.inputs
            .insert(self.next_input_index, (out.node_id, out.pin));
        self.next_input_index += 1;
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
    pub fn with_sharedata(mut self, val: DopTargetrelSharedata) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for DopTargetrel {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "targetrel"
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

pub trait DopTargetrelOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopTargetrelOutputs for DopTargetrel {}
impl DopTargetrelOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopTargetrel> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTerrainobjectMode {
    RayIntersect = 0,
    MetaBalls = 1,
    ImplicitBox = 2,
    ImplicitSphere = 3,
    ImplicitPlane = 4,
    Minimum = 5,
    VolumeSample = 6,
    Heightfield = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTerrainobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTerrainobjectVolumefilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTerrainobjectSurfreptype {
    Points = 0,
    Edges = 1,
}

#[derive(Debug, Clone)]
pub struct DopTerrainobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopTerrainobject {
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

    pub fn with_createframe(mut self, val: f32) -> Self {
        self.params.insert(
            "createframe".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_createframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "createframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bounce(mut self, val: f32) -> Self {
        self.params.insert(
            "bounce".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounce".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bounceforward(mut self, val: f32) -> Self {
        self.params.insert(
            "bounceforward".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bounceforward_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bounceforward".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_friction(mut self, val: f32) -> Self {
        self.params.insert(
            "friction".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_friction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "friction".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dynamicfriction(mut self, val: f32) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_dynamicfriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dynamicfriction".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_temperature(mut self, val: f32) -> Self {
        self.params.insert(
            "temperature".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_temperature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "temperature".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_sweepalpha(mut self, val: f32) -> Self {
        self.params.insert(
            "sweepalpha".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sweepalpha_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepalpha".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_radius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_radius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_length(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_length".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_length".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_collision_margin(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_collision_margin".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_collision_margin_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_collision_margin".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_shrink_amount(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_shrink_amount".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_shrink_amount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_shrink_amount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_adjust_factor(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_adjust_factor".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_adjust_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_adjust_factor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_linear_sleep_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_linear_sleep_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_linear_sleep_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_linear_sleep_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_angular_sleep_threshold(mut self, val: f32) -> Self {
        self.params.insert(
            "bullet_angular_sleep_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bullet_angular_sleep_threshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_angular_sleep_threshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_polylod(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "polylod".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_polylod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polylod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_basenormal(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "basenormal".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_basenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "basenormal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_volumecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "volumecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_volumecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumecolor".to_string(),
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
    pub fn with_bullet_deactivated_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_deactivated_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_deactivated_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_deactivated_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_primt(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_primT".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_primt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_primT".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_primr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_primR".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_primr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_primR".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_prims(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "bullet_primS".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_bullet_prims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_primS".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_sweepcount(mut self, val: i32) -> Self {
        self.params.insert(
            "sweepcount".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_sweepcount_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sweepcount".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_mode(mut self, val: DopTerrainobjectMode) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_mode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uniformvoxels(mut self, val: DopTerrainobjectUniformvoxels) -> Self {
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
    pub fn with_volumefilemode(mut self, val: DopTerrainobjectVolumefilemode) -> Self {
        self.params.insert(
            "volumefilemode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_volumefilemode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumefilemode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_surfreptype(mut self, val: DopTerrainobjectSurfreptype) -> Self {
        self.params.insert(
            "surfreptype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_surfreptype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "surfreptype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_name(mut self, val: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_object_name_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "object_name".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "soppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objpath(mut self, val: &str) -> Self {
        self.params.insert(
            "objpath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_volumefile(mut self, val: &str) -> Self {
        self.params.insert(
            "volumefile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_volumefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "volumefile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_georep(mut self, val: &str) -> Self {
        self.params.insert(
            "bullet_georep".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_georep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_georep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usesimframe(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_heightfield(mut self, val: bool) -> Self {
        self.params.insert(
            "heightfield".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_heightfield_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "heightfield".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animategeo(mut self, val: bool) -> Self {
        self.params.insert(
            "animategeo".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animategeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetransform(mut self, val: bool) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usetransform_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetransform".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_active(mut self, val: bool) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
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
    pub fn with_displayextrusion(mut self, val: bool) -> Self {
        self.params.insert(
            "displayextrusion".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_displayextrusion_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displayextrusion".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usevolume(mut self, val: bool) -> Self {
        self.params.insert(
            "usevolume".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usevolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usevolume".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
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
    pub fn with_laserscan(mut self, val: bool) -> Self {
        self.params.insert(
            "laserscan".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_laserscan_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "laserscan".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fixsigns(mut self, val: bool) -> Self {
        self.params.insert(
            "fixsigns".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fixsigns_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fixsigns".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forcebounds(mut self, val: bool) -> Self {
        self.params.insert(
            "forcebounds".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_forcebounds_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcebounds".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_invert(mut self, val: bool) -> Self {
        self.params.insert(
            "invert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_invert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "invert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_polyconvert(mut self, val: bool) -> Self {
        self.params.insert(
            "polyconvert".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_polyconvert_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "polyconvert".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_triangulate(mut self, val: bool) -> Self {
        self.params.insert(
            "triangulate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_triangulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "triangulate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addbarycenter(mut self, val: bool) -> Self {
        self.params.insert(
            "addbarycenter".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addbarycenter_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addbarycenter".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showguide2(mut self, val: bool) -> Self {
        self.params.insert(
            "showguide2".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showguide2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showguide2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_geoconvexhull(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_geoconvexhull".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_geoconvexhull_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_geoconvexhull".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_groupconnected(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_groupconnected".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_groupconnected_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_groupconnected".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_autofit(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_autofit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_adjust_geometry(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_adjust_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_adjust_geometry_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_adjust_geometry".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_add_impact(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_add_impact".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_add_impact_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_add_impact".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bullet_want_deactivate(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_want_deactivate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_want_deactivate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_want_deactivate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopTerrainobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "terrainobject"
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

pub trait DopTerrainobjectOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 1"
    fn out_output11(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
    /// Output pin: "Output 2"
    fn out_output2(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output2".to_string()),
        }
    }
    /// Output pin: "Output 3"
    fn out_output3(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output3".to_string()),
        }
    }
}

impl DopTerrainobjectOutputs for DopTerrainobject {}
impl DopTerrainobjectOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopTerrainobject> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopThinplatecolliderSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopThinplatecollider {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopThinplatecollider {
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

    pub fn with_guidedepth(mut self, val: i32) -> Self {
        self.params.insert(
            "guidedepth".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guidedepth_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidedepth".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guideprim(mut self, val: i32) -> Self {
        self.params.insert(
            "guideprim".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_guideprim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guideprim".to_string(),
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
    pub fn with_sharedata(mut self, val: DopThinplatecolliderSharedata) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for DopThinplatecollider {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "thinplatecollider"
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

pub trait DopThinplatecolliderOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopThinplatecolliderOutputs for DopThinplatecollider {}
impl DopThinplatecolliderOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopThinplatecollider>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTopnetCheckpointformat {
    /// Python (Deprecated)
    PythonDeprecated = 0,
    Json = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTopnetCheckpointload {
    Never = 0,
    OnSceneLoad = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTopnetRegenerationtype {
    UpdateWorkItemsAndInvalidateCaches = 0,
    UpdateWorkItemsOnly = 1,
    IgnoreChanges = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTopnetEvaluationtime {
    NetworkCookTime = 0,
    GlobalStartTime = 1,
    Custom = 2,
}

#[derive(Debug, Clone)]
pub struct DopTopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopTopnet {
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

    pub fn trigger_generatestatic(mut self) -> Self {
        self.params.insert(
            "generatestatic".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cookbutton(mut self) -> Self {
        self.params.insert(
            "cookbutton".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_dirtybutton(mut self) -> Self {
        self.params.insert(
            "dirtybutton".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_cancelbutton(mut self) -> Self {
        self.params.insert(
            "cancelbutton".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_savetaskgraph(mut self) -> Self {
        self.params.insert(
            "savetaskgraph".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadtaskgraph(mut self) -> Self {
        self.params.insert(
            "loadtaskgraph".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_loadcheckpoint(mut self) -> Self {
        self.params.insert(
            "loadcheckpoint".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_taskgraphsaverate(mut self, val: i32) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_taskgraphsaverate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphsaverate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkpointrate(mut self, val: i32) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_checkpointrate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointrate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_customtime(mut self, val: i32) -> Self {
        self.params.insert(
            "customtime".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_customtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "customtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkpointformat(mut self, val: DopTopnetCheckpointformat) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointformat_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointformat".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkpointload(mut self, val: DopTopnetCheckpointload) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_checkpointload_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointload".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_regenerationtype(mut self, val: DopTopnetRegenerationtype) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_regenerationtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "regenerationtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_evaluationtime(mut self, val: DopTopnetEvaluationtime) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_evaluationtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "evaluationtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_taskgraphfile(mut self, val: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_taskgraphfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkpointfile(mut self, val: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkpointfile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointfile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler(mut self, val: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_topscheduler_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "topscheduler".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultlabel(mut self, val: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "defaultlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_taskgraphautosave(mut self, val: bool) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_taskgraphautosave_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "taskgraphautosave".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_checkpointenabled(mut self, val: bool) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_checkpointenabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "checkpointenabled".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savegraphattribs(mut self, val: bool) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savegraphattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savegraphattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedefaultlabel(mut self, val: bool) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedefaultlabel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedefaultlabel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_savescenefile(mut self, val: bool) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_savescenefile_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "savescenefile".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopTopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "topnet"
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTwostateconrelParmopMinstatetime {
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
pub enum DopTwostateconrelParmopCurrentstate {
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
pub enum DopTwostateconrelParmopForcecompare1 {
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
pub enum DopTwostateconrelForcecompare1 {
    DoesNotMatter = 0,
    MustBeLessThan = 1,
    MustBeGreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTwostateconrelParmopForcevalue1 {
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
pub enum DopTwostateconrelParmopDistancecompare1 {
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
pub enum DopTwostateconrelDistancecompare1 {
    DoesNotMatter = 0,
    MustBeLessThan = 1,
    MustBeGreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTwostateconrelParmopDistancevalue1 {
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
pub enum DopTwostateconrelParmopForcecompare0 {
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
pub enum DopTwostateconrelForcecompare0 {
    DoesNotMatter = 0,
    MustBeLessThan = 1,
    MustBeGreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTwostateconrelParmopForcevalue0 {
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
pub enum DopTwostateconrelParmopDistancecompare0 {
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
pub enum DopTwostateconrelDistancecompare0 {
    DoesNotMatter = 0,
    MustBeLessThan = 1,
    MustBeGreaterThan = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTwostateconrelParmopDistancevalue0 {
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
pub enum DopTwostateconrelDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopTwostateconrelSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopTwostateconrel {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<usize, (usize, houdini_ramen_core::types::OutputPin)>,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopTwostateconrel {
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
    pub fn set_data_to_attach_accepts_2_of_bullet_soft_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(1, (out.node_id, out.pin));
        self
    }
    pub fn set_data_to_attach_accepts_2_of_bullet_soft_1_input<
        O: Into<houdini_ramen_core::types::NodeOutput>,
    >(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
        self.inputs.insert(2, (out.node_id, out.pin));
        self
    }

    pub fn with_minstatetime(mut self, val: f32) -> Self {
        self.params.insert(
            "minstatetime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minstatetime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minstatetime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forcevalue1(mut self, val: f32) -> Self {
        self.params.insert(
            "forcevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcevalue1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancevalue1(mut self, val: f32) -> Self {
        self.params.insert(
            "distancevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancevalue1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forcevalue0(mut self, val: f32) -> Self {
        self.params.insert(
            "forcevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcevalue0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancevalue0(mut self, val: f32) -> Self {
        self.params.insert(
            "distancevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancevalue0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_currentstate(mut self, val: i32) -> Self {
        self.params.insert(
            "currentstate".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_currentstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "currentstate".to_string(),
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
    pub fn with_parmop_minstatetime(mut self, val: DopTwostateconrelParmopMinstatetime) -> Self {
        self.params.insert(
            "parmop_minstatetime".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_minstatetime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_minstatetime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_currentstate(mut self, val: DopTwostateconrelParmopCurrentstate) -> Self {
        self.params.insert(
            "parmop_currentstate".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_currentstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_currentstate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_forcecompare1(mut self, val: DopTwostateconrelParmopForcecompare1) -> Self {
        self.params.insert(
            "parmop_forcecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forcecompare1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forcecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forcecompare1(mut self, val: DopTwostateconrelForcecompare1) -> Self {
        self.params.insert(
            "forcecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_forcecompare1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_forcevalue1(mut self, val: DopTwostateconrelParmopForcevalue1) -> Self {
        self.params.insert(
            "parmop_forcevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forcevalue1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forcevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancecompare1(
        mut self,
        val: DopTwostateconrelParmopDistancecompare1,
    ) -> Self {
        self.params.insert(
            "parmop_distancecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancecompare1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancecompare1(mut self, val: DopTwostateconrelDistancecompare1) -> Self {
        self.params.insert(
            "distancecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distancecompare1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancecompare1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancevalue1(
        mut self,
        val: DopTwostateconrelParmopDistancevalue1,
    ) -> Self {
        self.params.insert(
            "parmop_distancevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancevalue1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancevalue1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_forcecompare0(mut self, val: DopTwostateconrelParmopForcecompare0) -> Self {
        self.params.insert(
            "parmop_forcecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forcecompare0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forcecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_forcecompare0(mut self, val: DopTwostateconrelForcecompare0) -> Self {
        self.params.insert(
            "forcecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_forcecompare0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_forcevalue0(mut self, val: DopTwostateconrelParmopForcevalue0) -> Self {
        self.params.insert(
            "parmop_forcevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forcevalue0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forcevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancecompare0(
        mut self,
        val: DopTwostateconrelParmopDistancecompare0,
    ) -> Self {
        self.params.insert(
            "parmop_distancecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancecompare0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancecompare0(mut self, val: DopTwostateconrelDistancecompare0) -> Self {
        self.params.insert(
            "distancecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_distancecompare0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancecompare0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_distancevalue0(
        mut self,
        val: DopTwostateconrelParmopDistancevalue0,
    ) -> Self {
        self.params.insert(
            "parmop_distancevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_distancevalue0_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_distancevalue0".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopTwostateconrelDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopTwostateconrelSharedata) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for DopTwostateconrel {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "twostateconrel"
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

pub trait DopTwostateconrelOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopTwostateconrelOutputs for DopTwostateconrel {}
impl DopTwostateconrelOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopTwostateconrel>
{
}
