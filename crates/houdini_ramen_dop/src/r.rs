#[derive(Debug, Clone)]
pub struct DopRbdangularconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdangularconstraint {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_erp(mut self, val: f32) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalrotation(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
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
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animategoalrotation(mut self, val: bool) -> Self {
        self.params.insert(
            "animategoalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategoalrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animategoalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
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
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdangularconstraint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdangularconstraint"
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

pub trait DopRbdangularconstraintOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdangularconstraintOutputs for DopRbdangularconstraint {}
impl DopRbdangularconstraintOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdangularconstraint>
{
}

#[derive(Debug, Clone)]
pub struct DopRbdangularspringconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdangularspringconstraint {
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

    pub fn set_input_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_strength(mut self, val: f32) -> Self {
        self.params.insert(
            "strength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "damping".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxtorque(mut self, val: f32) -> Self {
        self.params.insert(
            "maxtorque".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxtorque_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxtorque".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxrotation(mut self, val: f32) -> Self {
        self.params.insert(
            "maxrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalrotation(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restrotation(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "restrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_restrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
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
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animategoalrotation(mut self, val: bool) -> Self {
        self.params.insert(
            "animategoalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategoalrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animategoalrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limittorque(mut self, val: bool) -> Self {
        self.params.insert(
            "limittorque".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limittorque_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limittorque".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limitrotation(mut self, val: bool) -> Self {
        self.params.insert(
            "limitrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitrotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitrotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
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
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdangularspringconstraint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdangularspringconstraint"
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

pub trait DopRbdangularspringconstraintOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdangularspringconstraintOutputs for DopRbdangularspringconstraint {}
impl DopRbdangularspringconstraintOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdangularspringconstraint>
{
}

#[derive(Debug, Clone)]
pub struct DopRbdautofreeze {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdautofreeze {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_angvelscale(mut self, val: f32) -> Self {
        self.params.insert(
            "angvelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angvelscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvelscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motiondecay(mut self, val: f32) -> Self {
        self.params.insert(
            "motiondecay".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motiondecay_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motiondecay".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "enablethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_enablethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_freezethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "freezethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_freezethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "freezethreshold".to_string(),
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
    pub fn with_melt(mut self, val: i32) -> Self {
        self.params.insert(
            "melt".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_melt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "melt".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdautofreeze {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdautofreeze"
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

pub trait DopRbdautofreezeOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdautofreezeOutputs for DopRbdautofreeze {}
impl DopRbdautofreezeOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdautofreeze> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdcarfollowpathGround {
    None = 0,
    GroundPlane = 1,
    HeightField = 2,
}

#[derive(Debug, Clone)]
pub struct DopRbdcarfollowpath {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdcarfollowpath {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_curvepos(mut self, val: f32) -> Self {
        self.params.insert(
            "curvepos".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_curvepos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "curvepos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drift(mut self, val: f32) -> Self {
        self.params.insert(
            "drift".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_drift_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drift".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overshoot(mut self, val: f32) -> Self {
        self.params.insert(
            "overshoot".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_overshoot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overshoot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_airresist(mut self, val: f32) -> Self {
        self.params.insert(
            "airresist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_airresist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "airresist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drag(mut self, val: f32) -> Self {
        self.params.insert(
            "drag".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_drag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_range_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "range_f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_range_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "range_f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_groundpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groundpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_groundrot(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "groundrot".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_groundrot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groundrot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enable(mut self, val: i32) -> Self {
        self.params.insert(
            "enable".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_enable_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enable".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_response(mut self, val: i32) -> Self {
        self.params.insert(
            "response".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_response_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "response".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ground(mut self, val: DopRbdcarfollowpathGround) -> Self {
        self.params.insert(
            "ground".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_ground_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ground".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_path(mut self, val: &str) -> Self {
        self.params.insert(
            "path".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_path_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "path".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_heightfield(mut self, val: &str) -> Self {
        self.params.insert(
            "heightfield".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
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
    pub fn with_dospeedlimit(mut self, val: bool) -> Self {
        self.params.insert(
            "dospeedlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_dospeedlimit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dospeedlimit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdcarfollowpath {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdcarfollowpath"
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

pub trait DopRbdcarfollowpathOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdcarfollowpathOutputs for DopRbdcarfollowpath {}
impl DopRbdcarfollowpathOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdcarfollowpath>
{
}

#[derive(Debug, Clone)]
pub struct DopRbdconetwistconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdconetwistconstraint {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_max_up_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "max_up_rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_up_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_up_rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_max_out_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "max_out_rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_out_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_out_rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_max_twist(mut self, val: f32) -> Self {
        self.params.insert(
            "max_twist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_twist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_twist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "softness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "softness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positionlimitstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "positionlimitstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionlimitstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionlimitstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positionlimitdampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "positionlimitdampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionlimitdampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionlimitdampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angularlimitstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "angularlimitstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angularlimitstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angularlimitstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angularlimitdampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "angularlimitdampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angularlimitdampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angularlimitdampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_bias_factor(mut self, val: f32) -> Self {
        self.params.insert(
            "bias_factor".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_bias_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bias_factor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_relaxation_factor(mut self, val: f32) -> Self {
        self.params.insert(
            "relaxation_factor".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_relaxation_factor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "relaxation_factor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positioncfm(mut self, val: f32) -> Self {
        self.params.insert(
            "positioncfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positioncfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positioncfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_positionerp(mut self, val: f32) -> Self {
        self.params.insert(
            "positionerp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_positionerp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "positionerp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goal_twist_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "goal_twist_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_goal_twist_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_twist_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constrained_twist_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "constrained_twist_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constrained_twist_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_twist_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_targetangularstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_targetangularstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_targetangularstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetangularstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_targetangulardampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_targetangulardampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_targetangulardampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetangulardampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_maximpulse(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_maximpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_maximpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_maximpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_erp(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_erp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_erp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "motor_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_motor_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guide_size(mut self, val: f32) -> Self {
        self.params.insert(
            "guide_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guide_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goal_twist_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goal_twist_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goal_twist_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_twist_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goal_up_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goal_up_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goal_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_up_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constrained_twist_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "constrained_twist_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_constrained_twist_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_twist_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constrained_up_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "constrained_up_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_constrained_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_up_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_targetr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "motor_targetr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_motor_targetr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetr".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_targetprevr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "motor_targetprevr".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_motor_targetprevr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetprevr".to_string(),
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
    pub fn with_guide_twistlimitcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_twistlimitcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_twistlimitcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_twistlimitcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_secondary_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "secondary_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_secondary_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondary_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guide_twistaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_twistaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_twistaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_twistaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guide_upaxiscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guide_upaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guide_upaxiscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_upaxiscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_conetwistconrel_guide_motortargetcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "conetwistconrel_guide_motortargetcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_conetwistconrel_guide_motortargetcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conetwistconrel_guide_motortargetcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
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
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeinitialerror(mut self, val: bool) -> Self {
        self.params.insert(
            "computeinitialerror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeinitialerror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeinitialerror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enablesoft(mut self, val: bool) -> Self {
        self.params.insert(
            "enablesoft".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enablesoft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enablesoft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_enabled(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_enabled".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_enabled_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_enabled".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_targetcurrentpose(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_targetcurrentpose".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_targetcurrentpose_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_targetcurrentpose".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_hastargetprev(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_hastargetprev".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_hastargetprev_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_hastargetprev".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_motor_normalizemaximpulse(mut self, val: bool) -> Self {
        self.params.insert(
            "motor_normalizemaximpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_motor_normalizemaximpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "motor_normalizemaximpulse".to_string(),
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
    pub fn with_guide_show_soft(mut self, val: bool) -> Self {
        self.params.insert(
            "guide_show_soft".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_guide_show_soft_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_show_soft".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdconetwistconstraint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdconetwistconstraint"
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

pub trait DopRbdconetwistconstraintOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdconetwistconstraintOutputs for DopRbdconetwistconstraint {}
impl DopRbdconetwistconstraintOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdconetwistconstraint>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdconfigureobjectMode {
    RayIntersect = 0,
    MetaBalls = 1,
    ImplicitBox = 2,
    ImplicitSphere = 3,
    ImplicitPlane = 4,
    Minimum = 5,
    VolumeSample = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdconfigureobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdconfigureobjectVolumefilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdconfigureobjectSurfreptype {
    Points = 0,
    Edges = 1,
}

#[derive(Debug, Clone)]
pub struct DopRbdconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdconfigureobject {
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

    pub fn set_input_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_gluethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueimpulsehalflife(mut self, val: f32) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
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
    pub fn with_sdftol(mut self, val: f32) -> Self {
        self.params.insert(
            "sdftol".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdftol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdftol".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
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
    pub fn with_mode(mut self, val: DopRbdconfigureobjectMode) -> Self {
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
    pub fn with_uniformvoxels(mut self, val: DopRbdconfigureobjectUniformvoxels) -> Self {
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
    pub fn with_volumefilemode(mut self, val: DopRbdconfigureobjectVolumefilemode) -> Self {
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
    pub fn with_surfreptype(mut self, val: DopRbdconfigureobjectSurfreptype) -> Self {
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
    pub fn with_glueobject(mut self, val: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxyvolume(mut self, val: &str) -> Self {
        self.params.insert(
            "proxyvolume".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxyvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxyvolume".to_string(),
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
    pub fn with_inheritvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inheritvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
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
    pub fn with_computecom(mut self, val: bool) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdconfigureobject"
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

pub trait DopRbdconfigureobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdconfigureobjectOutputs for DopRbdconfigureobject {}
impl DopRbdconfigureobjectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdconfigureobject>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdfracturedobjectMode {
    RayIntersect = 0,
    MetaBalls = 1,
    ImplicitBox = 2,
    ImplicitSphere = 3,
    ImplicitPlane = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdfracturedobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdfracturedobjectVolumefilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdfracturedobjectSurfreptype {
    Points = 0,
    Edges = 1,
}

#[derive(Debug, Clone)]
pub struct DopRbdfracturedobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdfracturedobject {
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
    pub fn with_gluethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueimpulsehalflife(mut self, val: f32) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
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
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
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
    pub fn with_mode(mut self, val: DopRbdfracturedobjectMode) -> Self {
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
    pub fn with_uniformvoxels(mut self, val: DopRbdfracturedobjectUniformvoxels) -> Self {
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
    pub fn with_volumefilemode(mut self, val: DopRbdfracturedobjectVolumefilemode) -> Self {
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
    pub fn with_surfreptype(mut self, val: DopRbdfracturedobjectSurfreptype) -> Self {
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
    pub fn with_groupmask(mut self, val: &str) -> Self {
        self.params.insert(
            "groupmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_groupmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groupmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject(mut self, val: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
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
    pub fn with_fracturebyname(mut self, val: bool) -> Self {
        self.params.insert(
            "fracturebyname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fracturebyname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturebyname".to_string(),
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
    pub fn with_letsopsinterpolate(mut self, val: bool) -> Self {
        self.params.insert(
            "letsopsinterpolate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_letsopsinterpolate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "letsopsinterpolate".to_string(),
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
    pub fn with_inheritvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inheritvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
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
    pub fn with_geo_triangulate(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_triangulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
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
    pub fn with_computecom(mut self, val: bool) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdfracturedobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdfracturedobject"
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

pub trait DopRbdfracturedobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdfracturedobjectOutputs for DopRbdfracturedobject {}
impl DopRbdfracturedobjectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdfracturedobject>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdglueobjectMode {
    RayIntersect = 0,
    MetaBalls = 1,
    ImplicitBox = 2,
    ImplicitSphere = 3,
    ImplicitPlane = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdglueobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdglueobjectVolumefilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdglueobjectSurfreptype {
    Points = 0,
    Edges = 1,
}

#[derive(Debug, Clone)]
pub struct DopRbdglueobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdglueobject {
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
    pub fn with_gluethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueimpulsehalflife(mut self, val: f32) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_internal_gluethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "internal_gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_internal_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "internal_gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_internal_glueimpulsehalflife(mut self, val: f32) -> Self {
        self.params.insert(
            "internal_glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_internal_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "internal_glueimpulsehalflife".to_string(),
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
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
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
    pub fn with_mode(mut self, val: DopRbdglueobjectMode) -> Self {
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
    pub fn with_uniformvoxels(mut self, val: DopRbdglueobjectUniformvoxels) -> Self {
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
    pub fn with_volumefilemode(mut self, val: DopRbdglueobjectVolumefilemode) -> Self {
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
    pub fn with_surfreptype(mut self, val: DopRbdglueobjectSurfreptype) -> Self {
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
    pub fn with_objname(mut self, val: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objname".to_string(),
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
    pub fn with_groupmask(mut self, val: &str) -> Self {
        self.params.insert(
            "groupmask".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_groupmask_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "groupmask".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject(mut self, val: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
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
    pub fn with_fracturebyname(mut self, val: bool) -> Self {
        self.params.insert(
            "fracturebyname".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fracturebyname_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturebyname".to_string(),
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
    pub fn with_inheritvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inheritvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
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
    pub fn with_computecom(mut self, val: bool) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdglueobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdglueobject"
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

pub trait DopRbdglueobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdglueobjectOutputs for DopRbdglueobject {}
impl DopRbdglueobjectOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdglueobject> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdguideMethod {
    Velocity = 0,
    TargetVelocity = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdguideGuidesource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdguideGeosource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone)]
pub struct DopRbdguide {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdguide {
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

    pub fn set_input_stream_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_airresist(mut self, val: f32) -> Self {
        self.params.insert(
            "airresist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_airresist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "airresist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_drag(mut self, val: f32) -> Self {
        self.params.insert(
            "drag".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_drag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "drag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_blend(mut self, val: f32) -> Self {
        self.params.insert(
            "blend".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blend_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blend".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_linearthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "linearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_linearthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "linearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angularthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "angularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_angularthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_distancethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_distancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "distancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_accumulatedlinearthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "accumulatedlinearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accumulatedlinearthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accumulatedlinearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_accumulatedangularthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "accumulatedangularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accumulatedangularthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accumulatedangularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_accumulateddistancethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "accumulateddistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_accumulateddistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "accumulateddistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidedcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidedcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidedcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidedcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_unguidedcolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "unguidedcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_unguidedcolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "unguidedcolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_method(mut self, val: DopRbdguideMethod) -> Self {
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
    pub fn with_minneighbors(mut self, val: i32) -> Self {
        self.params.insert(
            "minneighbors".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minneighbors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minneighbors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidesource(mut self, val: DopRbdguideGuidesource) -> Self {
        self.params.insert(
            "guidesource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_guidesource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidesource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geosource(mut self, val: DopRbdguideGeosource) -> Self {
        self.params.insert(
            "geosource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geosource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidesoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "guidesoppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidesoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidesoppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geosoppath(mut self, val: &str) -> Self {
        self.params.insert(
            "geosoppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geosoppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosoppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableguiding(mut self, val: bool) -> Self {
        self.params.insert(
            "enableguiding".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableguiding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableguiding".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_enableunguiding(mut self, val: bool) -> Self {
        self.params.insert(
            "enableunguiding".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableunguiding_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableunguiding".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uselinearthreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "uselinearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_uselinearthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uselinearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useangularthreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "useangularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useangularthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useangularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedistancethreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usedistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useaccumulatedlinearthreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "useaccumulatedlinearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaccumulatedlinearthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useaccumulatedlinearthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useaccumulatedangularthreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "useaccumulatedangularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaccumulatedangularthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useaccumulatedangularthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useaccumulateddistancethreshold(mut self, val: bool) -> Self {
        self.params.insert(
            "useaccumulateddistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useaccumulateddistancethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useaccumulateddistancethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useneighbors(mut self, val: bool) -> Self {
        self.params.insert(
            "useneighbors".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useneighbors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useneighbors".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdguide {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdguide"
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

pub trait DopRbdguideOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output"
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

impl DopRbdguideOutputs for DopRbdguide {}
impl DopRbdguideOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdguide> {}

#[derive(Debug, Clone)]
pub struct DopRbdhingeconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdhingeconstraint {
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

    pub fn set_input_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_length(mut self, val: f32) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_erp(mut self, val: f32) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_condir(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_condir_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "condir".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
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
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
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
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdhingeconstraint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdhingeconstraint"
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

pub trait DopRbdhingeconstraintOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdhingeconstraintOutputs for DopRbdhingeconstraint {}
impl DopRbdhingeconstraintOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdhingeconstraint>
{
}

#[derive(Debug, Clone)]
pub struct DopRbdkeyactive {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdkeyactive {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn trigger_copylastsimulated(mut self) -> Self {
        self.params.insert(
            "copylastsimulated".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_copylast(mut self) -> Self {
        self.params.insert(
            "copylast".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_active(mut self, val: f32) -> Self {
        self.params.insert(
            "active".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
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
    pub fn with_blendsim(mut self, val: f32) -> Self {
        self.params.insert(
            "blendsim".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_blendsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "blendsim".to_string(),
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
    pub fn with_useobjpath(mut self, val: i32) -> Self {
        self.params.insert(
            "useobjpath".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_useobjpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useobjpath".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdkeyactive {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdkeyactive"
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

pub trait DopRbdkeyactiveOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdkeyactiveOutputs for DopRbdkeyactive {}
impl DopRbdkeyactiveOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdkeyactive> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdobjectMode {
    RayIntersect = 0,
    MetaBalls = 1,
    ImplicitBox = 2,
    ImplicitSphere = 3,
    ImplicitPlane = 4,
    Minimum = 5,
    VolumeSample = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdobjectVolumefilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdobjectSurfreptype {
    Points = 0,
    Edges = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdobjectObjtype {
    Box = 0,
    Sphere = 1,
    /// Cylinder (limited)
    CylinderLimited = 2,
    Capsule = 3,
    Composite = 4,
}

#[derive(Debug, Clone)]
pub struct DopRbdobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdobject {
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
    pub fn with_gluethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueimpulsehalflife(mut self, val: f32) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
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
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_length(mut self, val: f32) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
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
    pub fn with_primt(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "primT".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_primt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primT".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "primR".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_primr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primR".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prims(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "primS".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_prims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primS".to_string(),
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
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
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
    pub fn with_objtype(mut self, val: DopRbdobjectObjtype) -> Self {
        self.params.insert(
            "objType".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_objtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objType".to_string(),
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
    pub fn with_mode(mut self, val: DopRbdobjectMode) -> Self {
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
    pub fn with_uniformvoxels(mut self, val: DopRbdobjectUniformvoxels) -> Self {
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
    pub fn with_volumefilemode(mut self, val: DopRbdobjectVolumefilemode) -> Self {
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
    pub fn with_surfreptype(mut self, val: DopRbdobjectSurfreptype) -> Self {
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
    pub fn with_glueobject(mut self, val: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxyvolume(mut self, val: &str) -> Self {
        self.params.insert(
            "proxyvolume".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_proxyvolume_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "proxyvolume".to_string(),
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
    pub fn with_comppath(mut self, val: &str) -> Self {
        self.params.insert(
            "comppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_comppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comppath".to_string(),
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
    pub fn with_inheritvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inheritvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
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
    pub fn with_bullet_showguide(mut self, val: bool) -> Self {
        self.params.insert(
            "bullet_showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_bullet_showguide_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "bullet_showguide".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geo_triangulate(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_triangulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
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
    pub fn with_enableode(mut self, val: bool) -> Self {
        self.params.insert(
            "enableode".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_odedisp(mut self, val: bool) -> Self {
        self.params.insert(
            "odeDisp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_odedisp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "odeDisp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autofit(mut self, val: bool) -> Self {
        self.params.insert(
            "autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autofit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computecom(mut self, val: bool) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdobject"
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

pub trait DopRbdobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdobjectOutputs for DopRbdobject {}
impl DopRbdobjectOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdobject> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpackedobjectInitialstate {
    CreateActiveObjects = 0,
    CreateStaticObjects = 1,
    CreateAnimatedStaticObjects = 2,
    CreateDeformingStaticObjects = 3,
    CreateDeformingActiveObjects = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpackedobjectGeosource {
    Sop = 0,
    FirstContextGeometry = 1,
    SecondContextGeometry = 2,
    ThirdContextGeometry = 3,
    FourthContextGeometry = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpackedobjectDisplaygeo {
    SimulationGeometry = 0,
    RenderGeometry = 1,
}

#[derive(Debug, Clone)]
pub struct DopRbdpackedobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdpackedobject {
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
    pub fn with_minactivationimpulse(mut self, val: f32) -> Self {
        self.params.insert(
            "minactivationimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_minactivationimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minactivationimpulse".to_string(),
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
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
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
    pub fn with_initialstate(mut self, val: DopRbdpackedobjectInitialstate) -> Self {
        self.params.insert(
            "initialstate".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_initialstate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialstate".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geosource(mut self, val: DopRbdpackedobjectGeosource) -> Self {
        self.params.insert(
            "geosource".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_geosource_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geosource".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_displaygeo(mut self, val: DopRbdpackedobjectDisplaygeo) -> Self {
        self.params.insert(
            "displaygeo".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_displaygeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "displaygeo".to_string(),
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
    pub fn with_packtype(mut self, val: &str) -> Self {
        self.params.insert(
            "packtype".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_packtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "packtype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overwriteattributes(mut self, val: &str) -> Self {
        self.params.insert(
            "overwriteattributes".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_overwriteattributes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overwriteattributes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rendergeopath(mut self, val: &str) -> Self {
        self.params.insert(
            "rendergeopath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rendergeopath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rendergeopath".to_string(),
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
    pub fn with_overwritefromsop(mut self, val: bool) -> Self {
        self.params.insert(
            "overwritefromsop".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_overwritefromsop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "overwritefromsop".to_string(),
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
    pub fn with_inheritvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inheritvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
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
    pub fn with_geo_triangulate(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_triangulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
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
    pub fn with_allowinitialoverlap(mut self, val: bool) -> Self {
        self.params.insert(
            "allowinitialoverlap".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_allowinitialoverlap_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "allowinitialoverlap".to_string(),
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
    pub fn with_setminactivationimpulse(mut self, val: bool) -> Self {
        self.params.insert(
            "setminactivationimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_setminactivationimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "setminactivationimpulse".to_string(),
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
    pub fn with_computecom(mut self, val: bool) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inheritpivot(mut self, val: bool) -> Self {
        self.params.insert(
            "inheritpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inheritpivot_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inheritpivot".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdpackedobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdpackedobject"
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

pub trait DopRbdpackedobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdpackedobjectOutputs for DopRbdpackedobject {}
impl DopRbdpackedobjectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdpackedobject>
{
}

#[derive(Debug, Clone)]
pub struct DopRbdpinconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdpinconstraint {
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

    pub fn set_input_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_erp(mut self, val: f32) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis1(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "axis1".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_axis1_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis1".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_axis2(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "axis2".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_axis2_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "axis2".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_targetw(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "targetw".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_targetw_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "targetw".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxangularimpulse(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "maxangularimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_maxangularimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxangularimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numangularmotors(mut self, val: i32) -> Self {
        self.params.insert(
            "numangularmotors".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numangularmotors_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numangularmotors".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
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
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animategoalpos(mut self, val: bool) -> Self {
        self.params.insert(
            "animategoalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategoalpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animategoalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
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
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdpinconstraint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdpinconstraint"
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

pub trait DopRbdpinconstraintOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdpinconstraintOutputs for DopRbdpinconstraint {}
impl DopRbdpinconstraintOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdpinconstraint>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsegeo {
    UsePointValue = 0,
    OverridePointValue = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUset {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUser {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsevel {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUseangvel {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUseglueobject {
    UsePointValue = 0,
    OverridePointValue = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsegluethreshold {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUseglueimpulsehalflife {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectMode {
    RayIntersect = 0,
    MetaBalls = 1,
    ImplicitBox = 2,
    ImplicitSphere = 3,
    ImplicitPlane = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUniformvoxels {
    NonSquare = 0,
    XAxis = 1,
    YAxis = 2,
    ZAxis = 3,
    MaxAxis = 4,
    BySize = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectVolumefilemode {
    Automatic = 0,
    ReadFiles = 1,
    WriteFiles = 2,
    NoOperation = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectSurfreptype {
    Points = 0,
    Edges = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectObjtype {
    Box = 0,
    Sphere = 1,
    /// Cylinder (limited)
    CylinderLimited = 2,
    Capsule = 3,
    Composite = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsep {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsedensity {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsemass {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUseinertialtensorstiffness {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsebounce {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsefriction {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsedynamicfriction {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdpointobjectUsetemperature {
    UsePointValue = 0,
    AddToPointValue = 1,
    OverridePointValue = 2,
}

#[derive(Debug, Clone)]
pub struct DopRbdpointobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdpointobject {
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
    pub fn with_gluethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueimpulsehalflife(mut self, val: f32) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
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
    pub fn with_sdftol(mut self, val: f32) -> Self {
        self.params.insert(
            "sdftol".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sdftol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sdftol".to_string(),
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
    pub fn with_radius(mut self, val: f32) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_radius_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "radius".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_length(mut self, val: f32) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_length_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "length".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
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
    pub fn with_primt(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "primT".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_primt_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primT".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_primr(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "primR".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_primr_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primR".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prims(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "primS".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_prims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "primS".to_string(),
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
    pub fn with_usegeo(mut self, val: DopRbdpointobjectUsegeo) -> Self {
        self.params.insert(
            "usegeo".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usegeo_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegeo".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_uset(mut self, val: DopRbdpointobjectUset) -> Self {
        self.params.insert(
            "uset".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_uset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "uset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_user(mut self, val: DopRbdpointobjectUser) -> Self {
        self.params.insert(
            "user".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_user_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "user".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usevel(mut self, val: DopRbdpointobjectUsevel) -> Self {
        self.params.insert(
            "usevel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usevel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usevel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useangvel(mut self, val: DopRbdpointobjectUseangvel) -> Self {
        self.params.insert(
            "useangvel".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_useangvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useangvel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useglueobject(mut self, val: DopRbdpointobjectUseglueobject) -> Self {
        self.params.insert(
            "useglueobject".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_useglueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useglueobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usegluethreshold(mut self, val: DopRbdpointobjectUsegluethreshold) -> Self {
        self.params.insert(
            "usegluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usegluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usegluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useglueimpulsehalflife(
        mut self,
        val: DopRbdpointobjectUseglueimpulsehalflife,
    ) -> Self {
        self.params.insert(
            "useglueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_useglueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useglueimpulsehalflife".to_string(),
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
    pub fn with_objtype(mut self, val: DopRbdpointobjectObjtype) -> Self {
        self.params.insert(
            "objType".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_objtype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objType".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usep(mut self, val: DopRbdpointobjectUsep) -> Self {
        self.params.insert(
            "usep".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedensity(mut self, val: DopRbdpointobjectUsedensity) -> Self {
        self.params.insert(
            "usedensity".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usedensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usemass(mut self, val: DopRbdpointobjectUsemass) -> Self {
        self.params.insert(
            "usemass".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useinertialtensorstiffness(
        mut self,
        val: DopRbdpointobjectUseinertialtensorstiffness,
    ) -> Self {
        self.params.insert(
            "useinertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_useinertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useinertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usebounce(mut self, val: DopRbdpointobjectUsebounce) -> Self {
        self.params.insert(
            "usebounce".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usebounce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usebounce".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usefriction(mut self, val: DopRbdpointobjectUsefriction) -> Self {
        self.params.insert(
            "usefriction".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usefriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usefriction".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usedynamicfriction(mut self, val: DopRbdpointobjectUsedynamicfriction) -> Self {
        self.params.insert(
            "usedynamicfriction".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usedynamicfriction_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usedynamicfriction".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usetemperature(mut self, val: DopRbdpointobjectUsetemperature) -> Self {
        self.params.insert(
            "usetemperature".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_usetemperature_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usetemperature".to_string(),
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
    pub fn with_mode(mut self, val: DopRbdpointobjectMode) -> Self {
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
    pub fn with_uniformvoxels(mut self, val: DopRbdpointobjectUniformvoxels) -> Self {
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
    pub fn with_volumefilemode(mut self, val: DopRbdpointobjectVolumefilemode) -> Self {
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
    pub fn with_surfreptype(mut self, val: DopRbdpointobjectSurfreptype) -> Self {
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
    pub fn with_geopath(mut self, val: &str) -> Self {
        self.params.insert(
            "geopath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_geopath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geopath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extraattribs(mut self, val: &str) -> Self {
        self.params.insert(
            "extraattribs".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_extraattribs_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "extraattribs".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject(mut self, val: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
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
    pub fn with_comppath(mut self, val: &str) -> Self {
        self.params.insert(
            "comppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_comppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "comppath".to_string(),
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
    pub fn with_nml(mut self, val: bool) -> Self {
        self.params.insert(
            "nml".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_nml_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "nml".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepscale(mut self, val: bool) -> Self {
        self.params.insert(
            "usepscale".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_adddeformhint(mut self, val: bool) -> Self {
        self.params.insert(
            "adddeformhint".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adddeformhint_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adddeformhint".to_string(),
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
    pub fn with_geo_triangulate(mut self, val: bool) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_geo_triangulate_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "geo_triangulate".to_string(),
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
    pub fn with_enableode(mut self, val: bool) -> Self {
        self.params.insert(
            "enableode".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_enableode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "enableode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_odedisp(mut self, val: bool) -> Self {
        self.params.insert(
            "odeDisp".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_odedisp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "odeDisp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_autofit(mut self, val: bool) -> Self {
        self.params.insert(
            "autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_autofit_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "autofit".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computecom(mut self, val: bool) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdpointobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdpointobject"
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

pub trait DopRbdpointobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdpointobjectOutputs for DopRbdpointobject {}
impl DopRbdpointobjectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdpointobject>
{
}

#[derive(Debug, Clone)]
pub struct DopRbdsliderconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdsliderconstraint {
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_max_rotation(mut self, val: f32) -> Self {
        self.params.insert(
            "max_rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_max_rotation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "max_rotation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goal_rot_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "goal_rot_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_goal_rot_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_rot_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constrained_rot_offset(mut self, val: f32) -> Self {
        self.params.insert(
            "constrained_rot_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constrained_rot_offset_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_rot_offset".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hinge_position_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "hinge_position_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hinge_position_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hinge_position_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hinge_position_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "hinge_position_damping".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hinge_position_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hinge_position_damping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hinge_position_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "hinge_position_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hinge_position_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hinge_position_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hinge_angle_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "hinge_angle_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hinge_angle_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hinge_angle_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hinge_angle_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "hinge_angle_damping".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hinge_angle_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hinge_angle_damping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_hinge_angle_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "hinge_angle_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_hinge_angle_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "hinge_angle_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ortho_position_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "ortho_position_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ortho_position_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ortho_position_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ortho_position_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "ortho_position_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ortho_position_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ortho_position_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ortho_angle_softness(mut self, val: f32) -> Self {
        self.params.insert(
            "ortho_angle_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ortho_angle_softness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ortho_angle_softness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ortho_angle_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "ortho_angle_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_ortho_angle_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ortho_angle_cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guide_size(mut self, val: f32) -> Self {
        self.params.insert(
            "guide_size".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guide_size_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guide_size".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_slide_range(mut self, val: [f32; 2]) -> Self {
        self.params.insert(
            "slide_range".to_string(),
            houdini_ramen_core::types::ParamValue::Float2(val),
        );
        self
    }
    pub fn with_slide_range_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "slide_range".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goal_hinge_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goal_hinge_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goal_hinge_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_hinge_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goal_up_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goal_up_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goal_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goal_up_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constrained_hinge_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "constrained_hinge_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_constrained_hinge_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_hinge_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constrained_up_axis(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "constrained_up_axis".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_constrained_up_axis_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constrained_up_axis".to_string(),
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
    pub fn with_secondary_color(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "secondary_color".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_secondary_color_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "secondary_color".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
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
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
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
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdsliderconstraint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdsliderconstraint"
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

pub trait DopRbdsliderconstraintOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdsliderconstraintOutputs for DopRbdsliderconstraint {}
impl DopRbdsliderconstraintOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdsliderconstraint>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdsolverParmopCollisioniterations {
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
pub enum DopRbdsolverParmopContactiterations {
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
pub enum DopRbdsolverParmopSubcontactiterations {
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
pub enum DopRbdsolverParmopShockpropagation {
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
pub enum DopRbdsolverParmopResolvepenetration {
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
pub enum DopRbdsolverParmopUsepointvelocity {
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
pub enum DopRbdsolverParmopUsevolumevelocity {
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
pub enum DopRbdsolverParmopAddimpacts {
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
pub enum DopRbdsolverParmopGlueignoresrestingobjects {
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
pub enum DopRbdsolverParmopCullmode {
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
pub enum DopRbdsolverCullmode {
    None = 0,
    Sphere = 1,
    OrientedBoundingBox = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdsolverParmopContactgroupmethod {
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
pub enum DopRbdsolverContactgroupmethod {
    None = 0,
    MostCentralPoint = 1,
    AveragePoint = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdsolverParmopContactgrouptol {
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
pub enum DopRbdsolverParmopMinimumsubsteps {
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
pub enum DopRbdsolverParmopMaximumsubsteps {
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
pub enum DopRbdsolverParmopCflcond {
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
pub enum DopRbdsolverDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone)]
pub struct DopRbdsolver {
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

impl DopRbdsolver {
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

    pub fn with_contactgrouptol(mut self, val: f32) -> Self {
        self.params.insert(
            "contactgrouptol".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contactgrouptol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contactgrouptol".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cflcond(mut self, val: f32) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cflcond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collisioniterations(mut self, val: i32) -> Self {
        self.params.insert(
            "collisioniterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_collisioniterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisioniterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contactiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "contactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_contactiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subcontactiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "subcontactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_subcontactiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subcontactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shockpropagation(mut self, val: i32) -> Self {
        self.params.insert(
            "shockpropagation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shockpropagation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shockpropagation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvepenetration(mut self, val: i32) -> Self {
        self.params.insert(
            "resolvepenetration".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolvepenetration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvepenetration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_minimumsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maximumsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "maximumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maximumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maximumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_collisioniterations(
        mut self,
        val: DopRbdsolverParmopCollisioniterations,
    ) -> Self {
        self.params.insert(
            "parmop_collisioniterations".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_collisioniterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_collisioniterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_contactiterations(
        mut self,
        val: DopRbdsolverParmopContactiterations,
    ) -> Self {
        self.params.insert(
            "parmop_contactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_contactiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_contactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_subcontactiterations(
        mut self,
        val: DopRbdsolverParmopSubcontactiterations,
    ) -> Self {
        self.params.insert(
            "parmop_subcontactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_subcontactiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_subcontactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_shockpropagation(mut self, val: DopRbdsolverParmopShockpropagation) -> Self {
        self.params.insert(
            "parmop_shockpropagation".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_shockpropagation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_shockpropagation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_resolvepenetration(
        mut self,
        val: DopRbdsolverParmopResolvepenetration,
    ) -> Self {
        self.params.insert(
            "parmop_resolvepenetration".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_resolvepenetration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_resolvepenetration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_usepointvelocity(mut self, val: DopRbdsolverParmopUsepointvelocity) -> Self {
        self.params.insert(
            "parmop_usepointvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usepointvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_usepointvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_usevolumevelocity(
        mut self,
        val: DopRbdsolverParmopUsevolumevelocity,
    ) -> Self {
        self.params.insert(
            "parmop_usevolumevelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_usevolumevelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_usevolumevelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_addimpacts(mut self, val: DopRbdsolverParmopAddimpacts) -> Self {
        self.params.insert(
            "parmop_addimpacts".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_addimpacts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_addimpacts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_glueignoresrestingobjects(
        mut self,
        val: DopRbdsolverParmopGlueignoresrestingobjects,
    ) -> Self {
        self.params.insert(
            "parmop_glueignoresrestingobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_glueignoresrestingobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_glueignoresrestingobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_cullmode(mut self, val: DopRbdsolverParmopCullmode) -> Self {
        self.params.insert(
            "parmop_cullmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cullmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cullmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cullmode(mut self, val: DopRbdsolverCullmode) -> Self {
        self.params.insert(
            "cullmode".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_cullmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cullmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_contactgroupmethod(
        mut self,
        val: DopRbdsolverParmopContactgroupmethod,
    ) -> Self {
        self.params.insert(
            "parmop_contactgroupmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_contactgroupmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_contactgroupmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contactgroupmethod(mut self, val: DopRbdsolverContactgroupmethod) -> Self {
        self.params.insert(
            "contactgroupmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_contactgroupmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contactgroupmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_contactgrouptol(mut self, val: DopRbdsolverParmopContactgrouptol) -> Self {
        self.params.insert(
            "parmop_contactgrouptol".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_contactgrouptol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_contactgrouptol".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_minimumsubsteps(mut self, val: DopRbdsolverParmopMinimumsubsteps) -> Self {
        self.params.insert(
            "parmop_minimumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_minimumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_minimumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_maximumsubsteps(mut self, val: DopRbdsolverParmopMaximumsubsteps) -> Self {
        self.params.insert(
            "parmop_maximumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_maximumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_maximumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_cflcond(mut self, val: DopRbdsolverParmopCflcond) -> Self {
        self.params.insert(
            "parmop_cflcond".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_cflcond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_cflcond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopRbdsolverDefaultparmop) -> Self {
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
    pub fn with_usepointvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "usepointvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepointvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepointvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usevolumevelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "usevolumevelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usevolumevelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usevolumevelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addimpacts(mut self, val: bool) -> Self {
        self.params.insert(
            "addimpacts".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addimpacts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addimpacts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueignoresrestingobjects(mut self, val: bool) -> Self {
        self.params.insert(
            "glueignoresrestingobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glueignoresrestingobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueignoresrestingobjects".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopRbdsolver {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdsolver"
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

pub trait DopRbdsolverOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopRbdsolverOutputs for DopRbdsolver {}
impl DopRbdsolverOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdsolver> {}

#[derive(Debug, Clone)]
pub struct DopRbdspringconstraint {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdspringconstraint {
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

    pub fn set_input_0_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_strength(mut self, val: f32) -> Self {
        self.params.insert(
            "strength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_strength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "strength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restlength(mut self, val: f32) -> Self {
        self.params.insert(
            "restlength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_damping(mut self, val: f32) -> Self {
        self.params.insert(
            "damping".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_damping_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "damping".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxforce(mut self, val: f32) -> Self {
        self.params.insert(
            "maxforce".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxforce".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxlength(mut self, val: f32) -> Self {
        self.params.insert(
            "maxlength".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guiderad(mut self, val: f32) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_guiderad_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guiderad".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_objectpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_objectpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "objectpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalpos(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_goalpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_guidecolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_guidecolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "guidecolor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "numiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiterations".to_string(),
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
    pub fn with_goalgroup(mut self, val: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_goalgroup_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "goalgroup".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animategoalpos(mut self, val: bool) -> Self {
        self.params.insert(
            "animategoalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animategoalpos_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animategoalpos".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mirror(mut self, val: bool) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mirror_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mirror".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limitforce(mut self, val: bool) -> Self {
        self.params.insert(
            "limitforce".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitforce_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitforce".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_limitlength(mut self, val: bool) -> Self {
        self.params.insert(
            "limitlength".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_limitlength_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "limitlength".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_disablecollisions(mut self, val: bool) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_disablecollisions_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "disablecollisions".to_string(),
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
    pub fn with_showobjectlink(mut self, val: bool) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_showobjectlink_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "showobjectlink".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRbdspringconstraint {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdspringconstraint"
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

pub trait DopRbdspringconstraintOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRbdspringconstraintOutputs for DopRbdspringconstraint {}
impl DopRbdspringconstraintOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdspringconstraint>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdstateParmopT {
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
pub enum DopRbdstateParmopR {
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
pub enum DopRbdstateParmopVel {
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
pub enum DopRbdstateParmopAngvel {
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
pub enum DopRbdstateParmopInheritvelocity {
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
pub enum DopRbdstateParmopGlueobject {
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
pub enum DopRbdstateParmopGluethreshold {
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
pub enum DopRbdstateParmopGlueimpulse {
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
pub enum DopRbdstateParmopGlueimpulsehalflife {
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
pub enum DopRbdstateParmopComputecom {
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
pub enum DopRbdstateParmopP {
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
pub enum DopRbdstateParmopComputemass {
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
pub enum DopRbdstateParmopDensity {
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
pub enum DopRbdstateParmopMass {
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
pub enum DopRbdstateParmopComputeinertialtensor {
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
pub enum DopRbdstateParmopComputeinertialtensortype {
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
pub enum DopRbdstateComputeinertialtensortype {
    UseVolume = 0,
    UseBoundingBox = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdstateParmopInertialtensorstiffness {
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
pub enum DopRbdstateDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdstateSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopRbdstate {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdstate {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_gluethreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueimpulse(mut self, val: f32) -> Self {
        self.params.insert(
            "glueimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glueimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueimpulsehalflife(mut self, val: f32) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_density(mut self, val: f32) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mass(mut self, val: f32) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_inertialtensorstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inertialtensorstiffness".to_string(),
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
    pub fn with_vel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_angvel(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "angvel".to_string(),
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
    pub fn with_parmop_t(mut self, val: DopRbdstateParmopT) -> Self {
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
    pub fn with_parmop_r(mut self, val: DopRbdstateParmopR) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_r_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_r".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_vel(mut self, val: DopRbdstateParmopVel) -> Self {
        self.params.insert(
            "parmop_vel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_vel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_vel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_angvel(mut self, val: DopRbdstateParmopAngvel) -> Self {
        self.params.insert(
            "parmop_angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_angvel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_angvel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_inheritvelocity(mut self, val: DopRbdstateParmopInheritvelocity) -> Self {
        self.params.insert(
            "parmop_inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_inheritvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_glueobject(mut self, val: DopRbdstateParmopGlueobject) -> Self {
        self.params.insert(
            "parmop_glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_glueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_gluethreshold(mut self, val: DopRbdstateParmopGluethreshold) -> Self {
        self.params.insert(
            "parmop_gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_gluethreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_gluethreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_glueimpulse(mut self, val: DopRbdstateParmopGlueimpulse) -> Self {
        self.params.insert(
            "parmop_glueimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_glueimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_glueimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_glueimpulsehalflife(
        mut self,
        val: DopRbdstateParmopGlueimpulsehalflife,
    ) -> Self {
        self.params.insert(
            "parmop_glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_glueimpulsehalflife_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_glueimpulsehalflife".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_computecom(mut self, val: DopRbdstateParmopComputecom) -> Self {
        self.params.insert(
            "parmop_computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_p(mut self, val: DopRbdstateParmopP) -> Self {
        self.params.insert(
            "parmop_p".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_p_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_p".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_computemass(mut self, val: DopRbdstateParmopComputemass) -> Self {
        self.params.insert(
            "parmop_computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_density(mut self, val: DopRbdstateParmopDensity) -> Self {
        self.params.insert(
            "parmop_density".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_density_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_density".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_mass(mut self, val: DopRbdstateParmopMass) -> Self {
        self.params.insert(
            "parmop_mass".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_mass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_mass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_computeinertialtensor(
        mut self,
        val: DopRbdstateParmopComputeinertialtensor,
    ) -> Self {
        self.params.insert(
            "parmop_computeinertialtensor".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_computeinertialtensor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_computeinertialtensor".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_computeinertialtensortype(
        mut self,
        val: DopRbdstateParmopComputeinertialtensortype,
    ) -> Self {
        self.params.insert(
            "parmop_computeinertialtensortype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_computeinertialtensortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_computeinertialtensortype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeinertialtensortype(
        mut self,
        val: DopRbdstateComputeinertialtensortype,
    ) -> Self {
        self.params.insert(
            "computeinertialtensortype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_computeinertialtensortype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeinertialtensortype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_inertialtensorstiffness(
        mut self,
        val: DopRbdstateParmopInertialtensorstiffness,
    ) -> Self {
        self.params.insert(
            "parmop_inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_inertialtensorstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_inertialtensorstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopRbdstateDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopRbdstateSharedata) -> Self {
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
    pub fn with_glueobject(mut self, val: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueobject".to_string(),
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
    pub fn with_inheritvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_inheritvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "inheritvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computecom(mut self, val: bool) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computecom_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computecom".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computemass(mut self, val: bool) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computemass_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computemass".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_computeinertialtensor(mut self, val: bool) -> Self {
        self.params.insert(
            "computeinertialtensor".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_computeinertialtensor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "computeinertialtensor".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopRbdstate {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdstate"
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

pub trait DopRbdstateOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopRbdstateOutputs for DopRbdstate {}
impl DopRbdstateOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdstate> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRbdvisualizationSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopRbdvisualization {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRbdvisualization {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_impactsscale(mut self, val: f32) -> Self {
        self.params.insert(
            "impactsscale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_impactsscale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactsscale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_impactscolor(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "impactscolor".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_impactscolor_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impactscolor".to_string(),
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
    pub fn with_sharedata(mut self, val: DopRbdvisualizationSharedata) -> Self {
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
    pub fn with_impacts(mut self, val: bool) -> Self {
        self.params.insert(
            "impacts".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_impacts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "impacts".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopRbdvisualization {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rbdvisualization"
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

pub trait DopRbdvisualizationOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopRbdvisualizationOutputs for DopRbdvisualization {}
impl DopRbdvisualizationOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRbdvisualization>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopReferenceframeforceParmopNewframeobject {
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
pub enum DopReferenceframeforceParmopNewframeposition {
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
pub enum DopReferenceframeforceParmopForcescale {
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
pub enum DopReferenceframeforceParmopSamplemode {
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
pub enum DopReferenceframeforceSamplemode {
    Default = 0,
    Point = 1,
    Circle = 2,
    Sphere = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopReferenceframeforceDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopReferenceframeforceSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopReferenceframeforce {
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

impl DopReferenceframeforce {
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

    pub fn with_forcescale(mut self, val: f32) -> Self {
        self.params.insert(
            "forcescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_forcescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "forcescale".to_string(),
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
    pub fn with_parmop_newframeobject(
        mut self,
        val: DopReferenceframeforceParmopNewframeobject,
    ) -> Self {
        self.params.insert(
            "parmop_newframeobject".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_newframeobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_newframeobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_newframeposition(
        mut self,
        val: DopReferenceframeforceParmopNewframeposition,
    ) -> Self {
        self.params.insert(
            "parmop_newframeposition".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_newframeposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_newframeposition".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_forcescale(mut self, val: DopReferenceframeforceParmopForcescale) -> Self {
        self.params.insert(
            "parmop_forcescale".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_forcescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_forcescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_samplemode(mut self, val: DopReferenceframeforceParmopSamplemode) -> Self {
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
    pub fn with_samplemode(mut self, val: DopReferenceframeforceSamplemode) -> Self {
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
    pub fn with_defaultparmop(mut self, val: DopReferenceframeforceDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopReferenceframeforceSharedata) -> Self {
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
    pub fn with_newframeobject(mut self, val: &str) -> Self {
        self.params.insert(
            "newframeobject".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_newframeobject_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newframeobject".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_newframeposition(mut self, val: &str) -> Self {
        self.params.insert(
            "newframeposition".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_newframeposition_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "newframeposition".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopReferenceframeforce {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "referenceframeforce"
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

pub trait DopReferenceframeforceOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopReferenceframeforceOutputs for DopReferenceframeforce {}
impl DopReferenceframeforceOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopReferenceframeforce>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRenderparmsParmopMaterialshop {
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
pub enum DopRenderparmsParmopDisplay {
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
pub enum DopRenderparmsParmopRender {
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
pub enum DopRenderparmsDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRenderparmsSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopRenderparms {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRenderparms {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
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
    pub fn with_render(mut self, val: i32) -> Self {
        self.params.insert(
            "render".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_render_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "render".to_string(),
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
    pub fn with_parmop_materialshop(mut self, val: DopRenderparmsParmopMaterialshop) -> Self {
        self.params.insert(
            "parmop_materialshop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_materialshop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_materialshop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_display(mut self, val: DopRenderparmsParmopDisplay) -> Self {
        self.params.insert(
            "parmop_display".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_render(mut self, val: DopRenderparmsParmopRender) -> Self {
        self.params.insert(
            "parmop_render".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_render_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_render".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopRenderparmsDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopRenderparmsSharedata) -> Self {
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
    pub fn with_materialshop(mut self, val: &str) -> Self {
        self.params.insert(
            "materialshop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_materialshop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materialshop".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopRenderparms {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "renderparms"
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

pub trait DopRenderparmsOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopRenderparmsOutputs for DopRenderparms {}
impl DopRenderparmsOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRenderparms> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRenderparmsvolatileParmopMaterialshop {
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
pub enum DopRenderparmsvolatileParmopDisplay {
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
pub enum DopRenderparmsvolatileParmopRender {
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
pub enum DopRenderparmsvolatileDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRenderparmsvolatileSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopRenderparmsvolatile {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRenderparmsvolatile {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
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
    pub fn with_render(mut self, val: i32) -> Self {
        self.params.insert(
            "render".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_render_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "render".to_string(),
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
    pub fn with_parmop_materialshop(
        mut self,
        val: DopRenderparmsvolatileParmopMaterialshop,
    ) -> Self {
        self.params.insert(
            "parmop_materialshop".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_materialshop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_materialshop".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_display(mut self, val: DopRenderparmsvolatileParmopDisplay) -> Self {
        self.params.insert(
            "parmop_display".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_display_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_display".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_render(mut self, val: DopRenderparmsvolatileParmopRender) -> Self {
        self.params.insert(
            "parmop_render".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_render_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_render".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopRenderparmsvolatileDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopRenderparmsvolatileSharedata) -> Self {
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
    pub fn with_materialshop(mut self, val: &str) -> Self {
        self.params.insert(
            "materialshop".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_materialshop_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "materialshop".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopRenderparmsvolatile {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "renderparmsvolatile"
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

pub trait DopRenderparmsvolatileOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopRenderparmsvolatileOutputs for DopRenderparmsvolatile {}
impl DopRenderparmsvolatileOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRenderparmsvolatile>
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRigidbodysolverSolver {
    Bullet = 0,
    Rbd = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRigidbodysolverConstraintsolvertype {
    /// Parallel Gauss-Seidel (Islands)
    ParallelGaussMinusSeidelIslands = 0,
    /// Parallel Gauss-Seidel (Graph Coloring)
    ParallelGaussMinusSeidelGraphColoring = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRigidbodysolverCullmode {
    None = 0,
    Sphere = 1,
    OrientedBoundingBox = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRigidbodysolverContactgroupmethod {
    None = 0,
    MostCentralPoint = 1,
    AveragePoint = 2,
}

#[derive(Debug, Clone)]
pub struct DopRigidbodysolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRigidbodysolver {
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

    pub fn set_objects_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_pre_solve_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input2".to_string()),
            (out.node_id, out.pin),
        );
        self
    }
    pub fn set_post_solve_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input3".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_timescale(mut self, val: f32) -> Self {
        self.params.insert(
            "timescale".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_timescale_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "timescale".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_sleepingtime(mut self, val: f32) -> Self {
        self.params.insert(
            "sleepingtime".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_sleepingtime_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "sleepingtime".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contactbreakingthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "contactbreakingthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contactbreakingthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contactbreakingthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraintsolvertolerance(mut self, val: f32) -> Self {
        self.params.insert(
            "constraintsolvertolerance".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_constraintsolvertolerance_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintsolvertolerance".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_globalcfm(mut self, val: f32) -> Self {
        self.params.insert(
            "globalcfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalcfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalcfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_globalerp(mut self, val: f32) -> Self {
        self.params.insert(
            "globalerp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_globalerp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "globalerp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_penetrationthreshold(mut self, val: f32) -> Self {
        self.params.insert(
            "penetrationthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_penetrationthreshold_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "penetrationthreshold".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_splitimpulseerp(mut self, val: f32) -> Self {
        self.params.insert(
            "splitimpulseerp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_splitimpulseerp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitimpulseerp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_erp(mut self, val: f32) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_erp_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "erp".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cfm(mut self, val: f32) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfm_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfm".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxomega(mut self, val: f32) -> Self {
        self.params.insert(
            "maxomega".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_maxomega_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxomega".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contactgrouptol(mut self, val: f32) -> Self {
        self.params.insert(
            "contactgrouptol".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_contactgrouptol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contactgrouptol".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractureminpiecevol(mut self, val: f32) -> Self {
        self.params.insert(
            "fractureminpiecevol".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fractureminpiecevol_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractureminpiecevol".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractureclusterfusedist(mut self, val: f32) -> Self {
        self.params.insert(
            "fractureclusterfusedist".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_fractureclusterfusedist_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractureclusterfusedist".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_converttopolylod(mut self, val: f32) -> Self {
        self.params.insert(
            "converttopolylod".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_converttopolylod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "converttopolylod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_substeps(mut self, val: i32) -> Self {
        self.params.insert(
            "substeps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_substeps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "substeps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_numiteration(mut self, val: i32) -> Self {
        self.params.insert(
            "numiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numiteration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numiteration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_oversample(mut self, val: i32) -> Self {
        self.params.insert(
            "oversample".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_oversample_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "oversample".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rand(mut self, val: i32) -> Self {
        self.params.insert(
            "rand".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_rand_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rand".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_minimumsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minimumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minimumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maximumsubsteps(mut self, val: i32) -> Self {
        self.params.insert(
            "maximumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maximumsubsteps_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maximumsubsteps".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cflcond(mut self, val: i32) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_cflcond_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cflcond".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_collisioniterations(mut self, val: i32) -> Self {
        self.params.insert(
            "collisioniterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_collisioniterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "collisioniterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contactiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "contactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_contactiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_subcontactiterations(mut self, val: i32) -> Self {
        self.params.insert(
            "subcontactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_subcontactiterations_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "subcontactiterations".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_shockpropagation(mut self, val: i32) -> Self {
        self.params.insert(
            "shockpropagation".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_shockpropagation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "shockpropagation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_resolvepenetration(mut self, val: i32) -> Self {
        self.params.insert(
            "resolvepenetration".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_resolvepenetration_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "resolvepenetration".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_cullmode(mut self, val: DopRigidbodysolverCullmode) -> Self {
        self.params.insert(
            "cullmode".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_cullmode_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cullmode".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_contactgroupmethod(mut self, val: DopRigidbodysolverContactgroupmethod) -> Self {
        self.params.insert(
            "contactgroupmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val as i32),
        );
        self
    }
    pub fn with_contactgroupmethod_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "contactgroupmethod".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractureseed(mut self, val: i32) -> Self {
        self.params.insert(
            "fractureseed".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_fractureseed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractureseed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_solver(mut self, val: DopRigidbodysolverSolver) -> Self {
        self.params.insert(
            "solver".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_solver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "solver".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_constraintsolvertype(
        mut self,
        val: DopRigidbodysolverConstraintsolvertype,
    ) -> Self {
        self.params.insert(
            "constraintsolvertype".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_constraintsolvertype_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "constraintsolvertype".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initialoverlaprel(mut self, val: &str) -> Self {
        self.params.insert(
            "initialoverlaprel".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initialoverlaprel_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initialoverlaprel".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyexternalconstraintnetwork(mut self, val: &str) -> Self {
        self.params.insert(
            "stickyexternalconstraintnetwork".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyexternalconstraintnetwork_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyexternalconstraintnetwork".to_string(),
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
    pub fn with_implicitdrag(mut self, val: bool) -> Self {
        self.params.insert(
            "implicitdrag".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_implicitdrag_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "implicitdrag".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doage(mut self, val: bool) -> Self {
        self.params.insert(
            "doage".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_doage_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doage".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_use_parallel_constraint_solver(mut self, val: bool) -> Self {
        self.params.insert(
            "use_parallel_constraint_solver".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_use_parallel_constraint_solver_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "use_parallel_constraint_solver".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_randomize_order(mut self, val: bool) -> Self {
        self.params.insert(
            "randomize_order".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_randomize_order_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "randomize_order".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_ensureindependentislands(mut self, val: bool) -> Self {
        self.params.insert(
            "ensureindependentislands".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_ensureindependentislands_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "ensureindependentislands".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_splitimpulse(mut self, val: bool) -> Self {
        self.params.insert(
            "splitimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_splitimpulse_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "splitimpulse".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_stickyuseinternalconstraintnetworks(mut self, val: bool) -> Self {
        self.params.insert(
            "stickyuseinternalconstraintnetworks".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_stickyuseinternalconstraintnetworks_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "stickyuseinternalconstraintnetworks".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usemaxomega(mut self, val: bool) -> Self {
        self.params.insert(
            "usemaxomega".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usemaxomega_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usemaxomega".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usepointvelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "usepointvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usepointvelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usepointvelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usevolumevelocity(mut self, val: bool) -> Self {
        self.params.insert(
            "usevolumevelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usevolumevelocity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usevolumevelocity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_addimpacts(mut self, val: bool) -> Self {
        self.params.insert(
            "addimpacts".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_addimpacts_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "addimpacts".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_glueignoresrestingobjects(mut self, val: bool) -> Self {
        self.params.insert(
            "glueignoresrestingobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_glueignoresrestingobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "glueignoresrestingobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fracturestampcreation(mut self, val: bool) -> Self {
        self.params.insert(
            "fracturestampcreation".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fracturestampcreation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturestampcreation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fracturefeedback(mut self, val: bool) -> Self {
        self.params.insert(
            "fracturefeedback".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fracturefeedback_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fracturefeedback".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_fractureignoresresting(mut self, val: bool) -> Self {
        self.params.insert(
            "fractureignoresresting".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_fractureignoresresting_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "fractureignoresresting".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_converttopoly(mut self, val: bool) -> Self {
        self.params.insert(
            "converttopoly".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_converttopoly_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "converttopoly".to_string(),
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

impl houdini_ramen_core::types::HoudiniNode for DopRigidbodysolver {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rigidbodysolver"
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

pub trait DopRigidbodysolverOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRigidbodysolverOutputs for DopRigidbodysolver {}
impl DopRigidbodysolverOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRigidbodysolver>
{
}

#[derive(Debug, Clone)]
pub struct DopRippleconfigureobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRippleconfigureobject {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
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
    pub fn with_rest_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "rest_soppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rest_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rest_soppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_animaterest(mut self, val: bool) -> Self {
        self.params.insert(
            "animaterest".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animaterest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animaterest".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for DopRippleconfigureobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rippleconfigureobject"
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

pub trait DopRippleconfigureobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRippleconfigureobjectOutputs for DopRippleconfigureobject {}
impl DopRippleconfigureobjectOutputs
    for houdini_ramen_core::graph::TypedExistingNodeRef<DopRippleconfigureobject>
{
}

#[derive(Debug, Clone)]
pub struct DopRippleobject {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRippleobject {
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
    pub fn with_numobjects(mut self, val: i32) -> Self {
        self.params.insert(
            "numobjects".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_numobjects_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "numobjects".to_string(),
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
    pub fn with_rest_soppath(mut self, val: &str) -> Self {
        self.params.insert(
            "rest_soppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rest_soppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rest_soppath".to_string(),
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
    pub fn with_animaterest(mut self, val: bool) -> Self {
        self.params.insert(
            "animaterest".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_animaterest_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "animaterest".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for DopRippleobject {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rippleobject"
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

pub trait DopRippleobjectOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRippleobjectOutputs for DopRippleobject {}
impl DopRippleobjectOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRippleobject> {}

#[derive(Debug, Clone)]
pub struct DopRipplesolver {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRipplesolver {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_cfl(mut self, val: f32) -> Self {
        self.params.insert(
            "cfl".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_cfl_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "cfl".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_conservation(mut self, val: f32) -> Self {
        self.params.insert(
            "conservation".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_conservation_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "conservation".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_wavespeed(mut self, val: f32) -> Self {
        self.params.insert(
            "wavespeed".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_wavespeed_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "wavespeed".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restspring(mut self, val: f32) -> Self {
        self.params.insert(
            "restspring".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_restspring_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restspring".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_minsubstep(mut self, val: i32) -> Self {
        self.params.insert(
            "minsubstep".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_minsubstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "minsubstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_maxsubstep(mut self, val: i32) -> Self {
        self.params.insert(
            "maxsubstep".to_string(),
            houdini_ramen_core::types::ParamValue::Int(val),
        );
        self
    }
    pub fn with_maxsubstep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "maxsubstep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdata(mut self, val: &str) -> Self {
        self.params.insert(
            "restdata".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_restdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "restdata".to_string(),
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
    pub fn with_adaptivestep(mut self, val: bool) -> Self {
        self.params.insert(
            "adaptivestep".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_adaptivestep_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "adaptivestep".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usenormal(mut self, val: bool) -> Self {
        self.params.insert(
            "usenormal".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usenormal_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usenormal".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_useedgelen(mut self, val: bool) -> Self {
        self.params.insert(
            "useedgelen".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_useedgelen_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "useedgelen".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_userestdata(mut self, val: bool) -> Self {
        self.params.insert(
            "userestdata".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_userestdata_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "userestdata".to_string(),
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
}

impl houdini_ramen_core::types::HoudiniNode for DopRipplesolver {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "ripplesolver"
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

pub trait DopRipplesolverOutputs: houdini_ramen_core::types::HoudiniNode {
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

impl DopRipplesolverOutputs for DopRipplesolver {}
impl DopRipplesolverOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRipplesolver> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRodmodeldataParmopRodstiffness {
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
pub enum DopRodmodeldataParmopRoddampingratio {
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
pub enum DopRodmodeldataParmopRodmassdensity {
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
pub enum DopRodmodeldataParmopRodthickness {
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
pub enum DopRodmodeldataParmopRodshapestiffness {
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
pub enum DopRodmodeldataDefaultparmop {
    SetInitial = 0,
    SetAlways = 1,
    SetNever = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRodmodeldataSharedata {
    DoNotShareData = 0,
    ShareDataAcrossAllTime = 1,
    ShareDataInOneTimestep = 2,
}

#[derive(Debug, Clone)]
pub struct DopRodmodeldata {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRodmodeldata {
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

    pub fn set_objects_to_be_processed_input<O: Into<houdini_ramen_core::types::NodeOutput>>(
        mut self,
        output: O,
    ) -> Self {
        let out = output.into();
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
        self.inputs.insert(
            houdini_ramen_core::types::InputPin::Name("input1".to_string()),
            (out.node_id, out.pin),
        );
        self
    }

    pub fn with_rodstiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_roddampingratio(mut self, val: f32) -> Self {
        self.params.insert(
            "roddampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_roddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "roddampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rodmassdensity(mut self, val: f32) -> Self {
        self.params.insert(
            "rodmassdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodmassdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rodthickness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodthickness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodthickness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_rodshapestiffness(mut self, val: f32) -> Self {
        self.params.insert(
            "rodshapestiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Float(val),
        );
        self
    }
    pub fn with_rodshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "rodshapestiffness".to_string(),
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
    pub fn with_parmop_rodstiffness(mut self, val: DopRodmodeldataParmopRodstiffness) -> Self {
        self.params.insert(
            "parmop_rodstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_rodstiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_rodstiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_roddampingratio(
        mut self,
        val: DopRodmodeldataParmopRoddampingratio,
    ) -> Self {
        self.params.insert(
            "parmop_roddampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_roddampingratio_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_roddampingratio".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_rodmassdensity(mut self, val: DopRodmodeldataParmopRodmassdensity) -> Self {
        self.params.insert(
            "parmop_rodmassdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_rodmassdensity_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_rodmassdensity".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_rodthickness(mut self, val: DopRodmodeldataParmopRodthickness) -> Self {
        self.params.insert(
            "parmop_rodthickness".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_rodthickness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_rodthickness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_parmop_rodshapestiffness(
        mut self,
        val: DopRodmodeldataParmopRodshapestiffness,
    ) -> Self {
        self.params.insert(
            "parmop_rodshapestiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_parmop_rodshapestiffness_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "parmop_rodshapestiffness".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_defaultparmop(mut self, val: DopRodmodeldataDefaultparmop) -> Self {
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
    pub fn with_sharedata(mut self, val: DopRodmodeldataSharedata) -> Self {
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

impl houdini_ramen_core::types::HoudiniNode for DopRodmodeldata {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rodmodeldata"
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

pub trait DopRodmodeldataOutputs: houdini_ramen_core::types::HoudiniNode {
    /// Output pin: "Output 1"
    fn out_output1(&self) -> houdini_ramen_core::types::NodeOutput {
        houdini_ramen_core::types::NodeOutput {
            node_id: self.get_id(),
            pin: houdini_ramen_core::types::OutputPin::Name("output1".to_string()),
        }
    }
}

impl DopRodmodeldataOutputs for DopRodmodeldata {}
impl DopRodmodeldataOutputs for houdini_ramen_core::graph::TypedExistingNodeRef<DopRodmodeldata> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRopDopTrange {
    RenderCurrentFrame = 0,
    RenderFrameRange = 1,
    /// Render Frame Range Only (Strict)
    RenderFrameRangeOnlyStrict = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DopRopDopCompresssims {
    NoCompression = 0,
    Blosc = 1,
    Gzip = 2,
}

#[derive(Debug, Clone)]
pub struct DopRopDop {
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

impl DopRopDop {
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

    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_executebackground(mut self) -> Self {
        self.params.insert(
            "executebackground".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn with_f(mut self, val: [f32; 3]) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Float3(val),
        );
        self
    }
    pub fn with_f_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "f".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_trange(mut self, val: DopRopDopTrange) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_trange_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "trange".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_compresssims(mut self, val: DopRopDopCompresssims) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            houdini_ramen_core::types::ParamValue::Menu(val as i32),
        );
        self
    }
    pub fn with_compresssims_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "compresssims".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_take(mut self, val: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_take_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "take".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doppath(mut self, val: &str) -> Self {
        self.params.insert(
            "doppath".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_doppath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "doppath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dopoutput(mut self, val: &str) -> Self {
        self.params.insert(
            "dopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_dopoutput_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "dopoutput".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prerender(mut self, val: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_prerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "prerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lprerender(mut self, val: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preframe(mut self, val: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_preframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "preframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpreframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postframe(mut self, val: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostframe(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postrender(mut self, val: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_postrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "postrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostrender(mut self, val: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::String(
                val.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_lpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "lpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_usesimframes(mut self, val: bool) -> Self {
        self.params.insert(
            "usesimframes".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_usesimframes_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "usesimframes".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_mkpath(mut self, val: bool) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_mkpath_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "mkpath".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_initsim(mut self, val: bool) -> Self {
        self.params.insert(
            "initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_initsim_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "initsim".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_alfprogress(mut self, val: bool) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_alfprogress_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "alfprogress".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tprerender(mut self, val: bool) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tprerender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tprerender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpreframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpreframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpreframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpostframe(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostframe_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostframe".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
    pub fn with_tpostrender(mut self, val: bool) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Toggle(val),
        );
        self
    }
    pub fn with_tpostrender_expr(mut self, expr: &str) -> Self {
        self.params.insert(
            "tpostrender".to_string(),
            houdini_ramen_core::types::ParamValue::Expression(
                expr.replace("\r\n", "\n").replace('\r', "\n"),
            ),
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRopDop {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "rop_dop"
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

#[derive(Debug, Clone)]
pub struct DopRopnet {
    pub id: usize,
    pub name: String,
    pub inputs: std::collections::BTreeMap<
        houdini_ramen_core::types::InputPin,
        (usize, houdini_ramen_core::types::OutputPin),
    >,
    pub params: std::collections::HashMap<String, houdini_ramen_core::types::ParamValue>,
    pub spare_params: Vec<houdini_ramen_core::types::SpareParam>,
}

impl DopRopnet {
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

    pub fn trigger_execute(mut self) -> Self {
        self.params.insert(
            "execute".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
    pub fn trigger_renderdialog(mut self) -> Self {
        self.params.insert(
            "renderdialog".to_string(),
            houdini_ramen_core::types::ParamValue::Button,
        );
        self
    }
}

impl houdini_ramen_core::types::HoudiniNode for DopRopnet {
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_node_type(&self) -> &'static str {
        "ropnet"
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
